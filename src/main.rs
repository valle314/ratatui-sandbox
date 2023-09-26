// reference: https://ratatui.rs/concepts/application-patterns/component-architecture.html

use std::{io, time::Duration};
use ratatui::{
    backend::CrosstermBackend,
    prelude,
    widgets,
    Terminal
};
use crossterm::{
    event,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::process::Command;

pub mod my_app;

enum InputMode {
    Normal,
    Editing,
}

struct App 
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


impl Default for App {
    fn default() -> App {
        App {
            counter: 0,
            should_quit: false,
            input: String::new(),
            input_mode: InputMode::Normal,
            cursor_position: 0,
        }
    }
}

impl App {
    fn increase_counter(&mut self)
    {
        self.counter += 1;
    }

    fn decrease_counter(&mut self)
    {
        self.counter -= 1;
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.input.push(new_char);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
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

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    fn submit_message(&mut self) {
        self.input.clear();
        self.reset_cursor();
    }
}


fn main() -> ()
{
    let mut terminal = setup_terminal();
    run(&mut terminal);
    restore_terminal(&mut terminal);
}

fn setup_terminal() -> Terminal<CrosstermBackend<io::Stdout>>
{
    let _ = enable_raw_mode();
    let mut stdout = io::stdout();
    let _ = execute!(stdout, EnterAlternateScreen);
    Terminal::new(CrosstermBackend::new(stdout)).unwrap()
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> ()
{
    let mut app = App::default();

    loop 
    {
        let _ = terminal.draw(|f| {
            render_app(&mut app, f);
        });

        update(&mut app);

        if app.should_quit 
        {
            break;
        }
    }
    return;
}

fn render_app(app: &mut App, frame: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>) 
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
        widgets::Paragraph::new(format!("counter is: {}", app.counter))
        .block(widgets::Block::default().borders(widgets::Borders::ALL).border_type(widgets::BorderType::Rounded)), chunks[0]);

    frame.render_widget(
        widgets::Paragraph::new(format!("string is: {}", app.input))
        .block(widgets::Block::default().borders(widgets::Borders::ALL).border_type(widgets::BorderType::Rounded)), chunks[1]);

    let input = widgets::Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => prelude::Style::default(),
            InputMode::Editing => prelude::Style::default().fg(prelude::Color::Yellow),
        })
        .block(widgets::Block::default().borders(widgets::Borders::ALL).title("Input"));
    frame.render_widget(input, chunks[2]);

    let output = Command::new("date-nlp").arg(&app.input).output().expect("failed to execute process");
    let date = String::from_utf8(output.stdout).unwrap();

    frame.render_widget(
        widgets::Paragraph::new(format!("THE DATE: {}", date))
        .block(widgets::Block::default().borders(widgets::Borders::ALL).border_type(widgets::BorderType::Rounded)), chunks[3]);
    frame.set_cursor(app.cursor_position.try_into().unwrap(), 10);
}

fn update(app: &mut App) -> ()
{
    if event::poll(Duration::from_millis(250)).is_ok() {
        if let event::Event::Key(key) = event::read().unwrap() 
        {
            match app.input_mode
            {
                InputMode::Normal => 
                {
                    match key.code
                    {
                        event::KeyCode::Char('j') => app.increase_counter(),
                        event::KeyCode::Char('k') => app.decrease_counter(),
                        event::KeyCode::Char('q') => app.should_quit = true,
                        event::KeyCode::Char('e') => app.input_mode = InputMode::Editing,
                        _ => (),
                    }
                }

                InputMode::Editing => 
                {
                    match key.code
                    {
                        event::KeyCode::Enter => 
                        {
                            app.submit_message();
                            app.input_mode = InputMode::Normal;
                        }

                        event::KeyCode::Char(to_insert) => 
                        {
                            app.enter_char(to_insert);
                        }

                        event::KeyCode::Backspace => 
                        {
                            app.delete_char();
                        }

                        event::KeyCode::Left => 
                        {
                            app.move_cursor_left();
                        }

                        event::KeyCode::Right => 
                        {
                            app.move_cursor_right();
                        }
                        _ => ()
                    }
                }
            }

        }
    }
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> ()
{
    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
}
