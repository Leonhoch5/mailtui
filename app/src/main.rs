mod ui;
mod mail;
mod token_store;
mod mail_oauth;
mod login_ui;
mod gmail;
mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::dotenv();

    app::run()?;
    Ok(())
}
