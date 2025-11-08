use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl,
    Scope, TokenResponse, TokenUrl, CsrfToken,
};
use oauth2::reqwest::http_client;
use tiny_http::Server;
use url::Url;


pub fn oauth_login(client_id: &str, client_secret: &str) -> Result<String, Box<dyn std::error::Error>> {
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;
    let redirect = RedirectUrl::new("http://127.0.0.1:8080/".to_string())?;

    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect);

    // use PKCE
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // request mail scope for IMAP access
    let (auth_url, _csrf_state) = client
        .authorize_url(|| CsrfToken::new_random())
        .add_scope(Scope::new("https://mail.google.com/".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    // open browser
    open::that(auth_url.as_str())?;

    // listen for redirect with code
    let server = Server::http("127.0.0.1:8080")?;
    let request = server.recv()?; // blocks until redirect arrives
    let full_url = format!("http://localhost{}", request.url());
    let parsed = Url::parse(&full_url)?;
    let code_pair = parsed
        .query_pairs()
        .find(|(k, _)| k == "code")
        .ok_or("no code in query")?;
    let code = AuthorizationCode::new(code_pair.1.into_owned());

    // reply to browser
    let response = tiny_http::Response::from_string("OK - you can close this window");
    let _ = request.respond(response);

    // exchange for token
    let token = client
        .exchange_code(code)
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier.secret().to_string()))
        .request(http_client)?;

    // return access token (use for XOAUTH2 with IMAP)
    Ok(token.access_token().secret().to_string())
}