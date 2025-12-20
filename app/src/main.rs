mod ui;
mod auth;
mod storage;
mod fetch;
mod token_store;
mod gmail;
mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::from_filename("app/.env").or_else(|_| dotenv::dotenv());

    app::run()?;
    Ok(())
}
