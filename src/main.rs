use std::io::Stdout;

use color_eyre::eyre::Result;
use ratatui::{crossterm, prelude::CrosstermBackend, widgets::Paragraph, Terminal};

pub mod event;
use event::{EventHandler, Message};

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

        let message = event_handler.next().await?;

        match message {
            Message::Key(key) => {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    if key.code == crossterm::event::KeyCode::Char('q') {
                        break;
                    }

                    if key.code == crossterm::event::KeyCode::Char('x') {
                        counter = 0;
                    }
                }
            }

            Message::Tick => counter += 1,

            _ => {}
        }
    }

    Ok(())
}
