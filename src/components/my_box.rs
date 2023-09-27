use std::io;
use ratatui::{ backend::CrosstermBackend, widgets, prelude };
use crossterm::event;

use crate::my_app;

impl my_app::Component for MyBox {
    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> bool
    {
        // match key_event.code
        // {
        //     event::KeyCode::Char('j') => self.my_fun(),
        //     _ => { return false; }
        // }
        return false;
    }

    fn render_app(&mut self, frame: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>) 
    {
        frame.render_widget(
            widgets::Paragraph::new(self.my_text.to_string())
            .block(widgets::Block::default().borders(widgets::Borders::ALL).border_type(widgets::BorderType::Rounded)), frame.size());
    }

    fn render_relative_app(&mut self, frame: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>, rect: ratatui::layout::Rect)
    {
        let chunks = prelude::Layout::default()
            .direction(prelude::Direction::Vertical)
            .constraints(
                [
                prelude::Constraint::Percentage(10),
                prelude::Constraint::Percentage(90)
                ].as_ref())
            .split(rect);

        frame.render_widget(
            widgets::Paragraph::new(self.my_text.to_string())
            .block(widgets::Block::default().borders(widgets::Borders::ALL).border_type(widgets::BorderType::Rounded)), chunks[0]);

        frame.render_widget(
            widgets::Paragraph::new("Some more text here lel")
            .block(widgets::Block::default().borders(widgets::Borders::ALL).border_type(widgets::BorderType::Rounded)), chunks[1]);
    }
}

pub struct MyBox
{
    my_text: String
}

impl MyBox
{
    pub fn new() -> Self
    {
        MyBox {
            my_text: "this is some long long long long and very long paragraph".to_string()
        }
    }

    pub fn my_fun(&mut self)
    {

    }
}
