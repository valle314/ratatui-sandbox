use std::{io, time::Duration};
use ratatui::{
    backend::CrosstermBackend,
    prelude,
    widgets,
    Terminal
};
use crossterm::event;
use std::process::Command;

use ratatui_sandbox::event_tui;

enum InputMode {
    Normal,
    Editing,
}

pub struct App 
{
    counter: i64,
    should_quit: bool,
    // currently typed text
    input: String,
    /// Position of cursor in the editor area.
    cursor_position: usize,
    /// Current input mode
    input_mode: InputMode
}


impl App {
    pub fn new() -> Self
    {
        App {
            counter: 0,
            should_quit: false,
            input: String::new(),
            input_mode: InputMode::Normal,
            cursor_position: 0,
        }
    }

    pub fn increase_counter(&mut self)
    {
        self.counter += 1;
    }

    pub fn decrease_counter(&mut self)
    {
        self.counter -= 1;
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.push(new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    pub fn submit_message(&mut self) {
        self.input.clear();
        self.reset_cursor();
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> ()
    {

        let events = event_tui::EventHandler::new(250);

        loop 
        {
            let _ = terminal.draw(|f| {
                self.render_app(f);
            });

            // self.update();

            match events.next() {
                event_tui::Event::Tick => self.tick(),
                event_tui::Event::Key(key_event) => self.handle_key_events(key_event),
                event_tui::Event::Mouse(_) => {}
                event_tui::Event::Resize(_, _) => {}
            }


            if self.should_quit 
            {
                break;
            }
        }
        return;
    }

    fn tick(&mut self)
    {

    }

    fn handle_key_events(&mut self, key_event: event::KeyEvent)
    {
        match key_event.code
        {
            event::KeyCode::Char('q') => self.should_quit = true,
            event::KeyCode::Char('c') => 
            {
                if key_event.modifiers == (event::KeyModifiers::CONTROL)
                {
                    self.should_quit = true
                }
            }
            _ => ()
        }

        // println!("key: {:?}", key_event);
    }

    fn render_app(&mut self, frame: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>) 
    {
        let chunks = prelude::Layout::default()
            .direction(prelude::Direction::Horizontal)
            .constraints(
                [
                prelude::Constraint::Percentage(10),
                prelude::Constraint::Percentage(20),
                prelude::Constraint::Percentage(30),
                prelude::Constraint::Percentage(40),
                ].as_ref())
            .split(frame.size());
        frame.render_widget(
            widgets::Paragraph::new(format!("counter is: {}", self.counter))
            .block(widgets::Block::default().borders(widgets::Borders::ALL).border_type(widgets::BorderType::Rounded)), chunks[0]);

        frame.render_widget(
            widgets::Paragraph::new(format!("string is: {}", self.input))
            .block(widgets::Block::default().borders(widgets::Borders::ALL).border_type(widgets::BorderType::Rounded)), chunks[1]);

        let input = widgets::Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => prelude::Style::default(),
                InputMode::Editing => prelude::Style::default().fg(prelude::Color::Yellow),
            })
        .block(widgets::Block::default().borders(widgets::Borders::ALL).title("Input"));
        frame.render_widget(input, chunks[2]);

        let output = Command::new("date-nlp").arg(&(self.input)).output().expect("failed to execute process");
        let date = String::from_utf8(output.stdout).unwrap();

        frame.render_widget(
            widgets::Paragraph::new(format!("THE DATE: {}", date))
            .block(widgets::Block::default().borders(widgets::Borders::ALL).border_type(widgets::BorderType::Rounded)), chunks[3]);
        frame.set_cursor(self.cursor_position.try_into().unwrap(), 10);
    }

    fn update(&mut self) -> ()
    {
        if event::poll(Duration::from_millis(250)).is_ok() {
            if let event::Event::Key(key) = event::read().unwrap() 
            {
                match self.input_mode
                {
                    InputMode::Normal => 
                    {
                        match key.code
                        {
                            event::KeyCode::Char('j') => self.increase_counter(),
                            event::KeyCode::Char('k') => self.decrease_counter(),
                            event::KeyCode::Char('q') => self.should_quit = true,
                            event::KeyCode::Char('e') => self.input_mode = InputMode::Editing,
                            _ => (),
                        }
                    }

                    InputMode::Editing => 
                    {
                        match key.code
                        {
                            event::KeyCode::Enter => 
                            {
                                self.submit_message();
                                self.input_mode = InputMode::Normal;
                            }

                            event::KeyCode::Char(to_insert) => 
                            {
                                self.enter_char(to_insert);
                            }

                            event::KeyCode::Backspace => 
                            {
                                self.delete_char();
                            }

                            event::KeyCode::Left => 
                            {
                                self.move_cursor_left();
                            }

                            event::KeyCode::Right => 
                            {
                                self.move_cursor_right();
                            }
                            _ => ()
                        }
                    }
                }

            }
        }
    }
}
