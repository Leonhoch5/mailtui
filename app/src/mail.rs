use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, RedirectUrl, Scope, TokenUrl, CsrfToken,
};
use open;


pub fn oauth_login(client_id: &str, _client_secret: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let redirect_str = std::env::var("MAIL_OAUTH_REDIRECT")
        .unwrap_or_else(|_| "https://mailtui.vercel.app/callback".to_string());
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;
    let redirect = RedirectUrl::new(redirect_str.clone())?;

    let client = oauth2::basic::BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new("".to_string())), 
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect);

    // build auth url; server-side will handle the callback and token exchange
    let (url, _state) = client
        .authorize_url(|| CsrfToken::new_random())
        .add_scope(Scope::new("https://mail.google.com/".to_string()))
        .add_extra_param("access_type", "offline")
        .add_extra_param("prompt", "consent")
        .url();

    // try to open browser, also print URL so user can use it manually
    println!("Open this URL to continue:\n{}", url.as_str());
    let _ = open::that(url.as_str());

    Ok(url.to_string())
}

use std::net::TcpStream;
use native_tls::TlsConnector;

/// sample stuff - will use oauth token (not implemented)
pub fn connect_oauth(_email: &str, _access_token: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("not implemented: XOAUTH2 authenticate".into())
}
