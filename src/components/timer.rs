use crate::{clock::Clock, components::component::Component};
use color_eyre::eyre::Result;
use ratatui::{layout::Rect, widgets::Paragraph, Frame};

#[derive(Default)]
pub struct TimerComponent {
    clock: Clock,
}

impl TimerComponent {
    pub fn new() -> Self {
        TimerComponent::default()
    }
}

impl Component for TimerComponent {
    fn handle_key_event(&mut self, event: crossterm::event::KeyEvent) -> Result<()> {
        if event.kind == crossterm::event::KeyEventKind::Press
            && event.code == crossterm::event::KeyCode::Char(' ')
        {
            self.clock.start();
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        frame.render_widget(Paragraph::new("Hello!"), area);

        Ok(())
    }
}
