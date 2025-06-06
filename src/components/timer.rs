use crate::{clock::Clock, components::component::Component, input::Action};
use color_eyre::eyre::Result;
use ratatui::{layout::Rect, widgets::Paragraph, Frame};

#[derive(Default)]
pub struct TimerComponent {
    clock: Clock,
    last_solve_time: f64,
}

impl TimerComponent {
    pub fn new() -> Self {
        TimerComponent::default()
    }
}

impl Component for TimerComponent {
    fn handle_action(&mut self, action: &Action) -> Result<()> {
        match action {
            // TODO: Add support for releasing space to start the timer
            // Due to console limitations this will only work on supported terminal emulators (eg. kitty)
            // Need to implement config system, this should be disabled by default.
            Action::TimerStartRelease => {}
            Action::TimerToggle => {
                if !self.clock.is_running() {
                    self.clock.start()
                } else if self.clock.is_running() && !self.clock.elapsed().is_zero() {
                    self.last_solve_time = self.clock.stop().as_secs_f64();
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let time = {
            if self.clock.is_running() {
                self.clock.elapsed().as_secs_f64()
            } else {
                self.last_solve_time
            }
        };

        frame.render_widget(Paragraph::new(time.to_string()), area);

        Ok(())
    }
}
