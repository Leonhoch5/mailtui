pub fn oauth_login(client_id: &str, client_secret: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    crate::mail::oauth_login(client_id, client_secret)
}
