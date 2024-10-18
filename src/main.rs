use color_eyre::eyre::Result;

pub mod app;
pub mod clock;
pub mod components;
pub mod event;

use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app: App = App::new();
    app.run().await?;

    Ok(())
}
