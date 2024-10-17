use std::io::Stdout;

use color_eyre::eyre::Result;
use ratatui::{crossterm, prelude::CrosstermBackend, widgets::Paragraph, Terminal};

pub mod app;
pub mod command;
pub mod components;
pub mod event;
pub mod timer;

use event::{Event, EventHandler};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let event_handler = EventHandler::new(5.0);
    let app_result = run(terminal, event_handler).await;

    ratatui::restore();

    app_result
}

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

async fn run(mut terminal: Tui, mut event_handler: event::EventHandler) -> Result<()> {
    let mut counter: u64 = 0;

    loop {
        let hello_message = String::from("Hello CubeGen! ");

        terminal.draw(|frame| {
            let message = hello_message + &counter.to_string();
            frame.render_widget(Paragraph::new(message), frame.area());
        })?;

        let event = event_handler.next().await?;

        match event {
            Event::Key(key) => {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    if key.code == crossterm::event::KeyCode::Char('q') {
                        break;
                    }

                    if key.code == crossterm::event::KeyCode::Char('x') {
                        counter = 0;
                    }
                }
            }

            Event::Tick => counter += 1,

            _ => {}
        }
    }

    Ok(())
}
