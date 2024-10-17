use crate::event::Event;
use color_eyre::eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{layout::Rect, Frame};

/// A component is a visual and interactive element of the UI.
/// Managed and updated by the main application loop
pub trait Component {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(key_event) => self.handle_key_event(key_event),
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
            Event::Tick => self.update(),

            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, _event: KeyEvent) -> Result<()> {
        Ok(())
    }

    fn handle_mouse_event(&mut self, _event: MouseEvent) -> Result<()> {
        Ok(())
    }

    // fn list_commands(&self) -> Vec<CommandInfo>;
}

pub trait DrawableComponent {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;
}
