use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenUrl, CsrfToken,
};
use oauth2::reqwest::http_client;
use oauth2::TokenResponse;
use crate::token_store::{SavedToken, save_token};
use open;
use tiny_http::Server;
use url::Url;
use std::net::TcpListener;


pub fn oauth_login(client_id: &str, _client_secret: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    let port = listener.local_addr()?.port();
    drop(listener);

    let redirect_str = format!("http://127.0.0.1:{}/", port);
    eprintln!("[mail] using redirect URI: {}", redirect_str);

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;
    let redirect = RedirectUrl::new(redirect_str.clone())?;

    // Use client secret when present, otherwise public client
    let client_secret_opt = if _client_secret.is_empty() {
        None
    } else {
        Some(ClientSecret::new(_client_secret.to_string()))
    };

    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        client_secret_opt,
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (url, _state) = client
        .authorize_url(|| CsrfToken::new_random())
        .add_scope(Scope::new("https://mail.google.com/".to_string()))
        .add_extra_param("access_type", "offline")
        .add_extra_param("prompt", "consent")
        .set_pkce_challenge(pkce_challenge)
        .url();

    let bind_addr = format!("127.0.0.1:{}", port);

    // try to bind the tiny_http server a few times
    let mut server = None;
    for attempt in 0..5 {
        match Server::http(&bind_addr) {
            Ok(s) => {
                server = Some(s);
                break;
            }
            Err(e) => {
                eprintln!("[mail] bind attempt {} failed: {}", attempt + 1, e);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
    let server = server.ok_or(format!("failed to bind to {} after retries", bind_addr))?;

    println!("Listening on http://{}", bind_addr);
    println!("Open this URL to continue:\n{}", url.as_str());
    if let Some(pair) = url.query_pairs().find(|(k, _)| k == "redirect_uri") {
        eprintln!("[mail] redirect_uri param in auth URL: {}", pair.1);
    }
    let _ = open::that(url.as_str());

    let request = server.recv()?;
    let full_url = format!("http://127.0.0.1:{}{}", port, request.url());
    eprintln!("[mail] received callback URL: {}", full_url);
    let parsed = Url::parse(&full_url)?;
    let code_pair = parsed
        .query_pairs()
        .find(|(k, _)| k == "code")
        .ok_or("no code in query")?;
    let code = AuthorizationCode::new(code_pair.1.into_owned());

    // build HTML echoing query params
    fn esc(s: &str) -> String {
        s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
    }

    let mut pairs_html = String::new();
    for (k, v) in parsed.query_pairs() {
        pairs_html.push_str(&format!(
            "<tr><td style=\"font-family:monospace\">{}</td><td style=\"font-family:monospace\">{}</td></tr>",
            esc(&k), esc(&v)
        ));
    }

    let html = format!(r#"<!doctype html>
<html>
  <head>
    <meta charset="utf-8"/>
    <title>MailTUI OAuth</title>
    <meta name="viewport" content="width=device-width,initial-scale=1"/>
    <style>
      body{{font-family:system-ui,Segoe UI,Roboto,Arial;padding:18px;color:#111}}
      table{{border-collapse:collapse;margin-top:12px}}
      td,th{{padding:6px;border:1px solid #ddd}}
      h1{{margin:0 0 8px 0;font-size:18px}}
    </style>
  </head>
  <body>
    <h1>OAuth callback received</h1>
    <p>Params received from Google (this window will close automatically):</p>
    <table>
      <thead><tr><th>Param</th><th>Value</th></tr></thead>
      <tbody>{}</tbody>
    </table>
    <p style="margin-top:12px"><small>If the window doesn't close, you can safely close it manually.</small></p>
    <script>setTimeout(()=>{{ try {{ window.close(); }} catch(e){{}} }}, 4000);</script>
  </body>
</html>"#, pairs_html);

    let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
    let response = tiny_http::Response::from_string(html).with_header(header);
    let _ = request.respond(response);

    // exchange for token
    let token = match client
        .exchange_code(code)
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier.secret().to_string()))
        .request(http_client)
    {
        Ok(t) => t,
        Err(e) => {
            eprintln!("[mail] token exchange failed: {:#?}", e);
            return Err(format!("token exchange failed: {}", e).into());
        }
    };

    let access = token.access_token().secret().to_string();

    // save token (convert expires_in to unix timestamp if available)
    let expires_at_unix = token.expires_in().map(|dur| {
        let now = std::time::SystemTime::now();
        let then = now + dur;
        then.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
    });

    let saved = SavedToken {
        access_token: token.access_token().secret().to_string(),
        refresh_token: token.refresh_token().map(|r| r.secret().to_string()),
        expires_at_unix,
    };
    let _ = save_token(&saved);

    // spawn background task to fetch mails and update the UI
    let access_clone = access.clone();
    std::thread::spawn(move || {
        match crate::gmail::fetch_latest(&access_clone, 10) {
            Ok(msgs) => {
                // populate UI messages
                crate::ui::set_messages(msgs);
            }
            Err(e) => eprintln!("failed to fetch gmail messages (bg): {}", e),
        }
    });

    Ok(access)
}

/// sample stuff - will use oauth token (not implemented)
pub fn connect_oauth(_email: &str, _access_token: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("not implemented: XOAUTH2 authenticate".into())
}
