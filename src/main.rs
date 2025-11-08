// module declarations
mod ui;
mod mail;
mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()?;
    Ok(())
}
