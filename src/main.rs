fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load .env into env vars for dev
    let _ = dotenv::dotenv();

    // ...existing code...
    app::run()?;
    Ok(())
}