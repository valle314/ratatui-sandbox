use std::{io, time::Duration};
use ratatui::{
    backend::CrosstermBackend,
    prelude,
    widgets,
    Terminal
};
use crossterm::event;
use std::process::Command;

use crate::event_tui;
use crate::components::my_paragraph;


// enum InputMode 
// {
//     Normal,
//     Editing,
// }

pub struct App 
{
    // counter: i64,
    should_quit: bool,
    // currently typed text
    // input: String,
    /// Position of cursor in the editor area.
    // cursor_position: usize,
    /// Current input mode
    // input_mode: InputMode
    component: my_paragraph::MyParagraph
}


impl App {
    pub fn new() -> Self
    {
        App {
            should_quit: false,
            component: my_paragraph::MyParagraph::new()
            // counter: 0,
            // input: String::new(),
            // input_mode: InputMode::Normal,
            // cursor_position: 0,
        }
    }


    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> ()
    {

        let events = event_tui::EventHandler::new(250);

        loop 
        {
            let _ = terminal.draw(|f| {
                self.component.render_app(f);
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
        // TODO send keys to all components!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        // and render things.. 
        // if one event is handled by one component then return false and then do not pass the
        // event to other components
        self.component.handle_key_event(key_event);

        match key_event.code
        {
            event::KeyCode::Char('q') => self.should_quit = true,
            event::KeyCode::Char('h') => my_paragraph::hi(),
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

    fn update(&mut self) -> ()
    {
        // if event::poll(Duration::from_millis(250)).is_ok() {
        //     if let event::Event::Key(key) = event::read().unwrap() 
        //     {
        //         match self.input_mode
        //         {
        //             InputMode::Normal => 
        //             {
        //                 match key.code
        //                 {
        //                     event::KeyCode::Char('j') => self.increase_counter(),
        //                     event::KeyCode::Char('k') => self.decrease_counter(),
        //                     event::KeyCode::Char('q') => self.should_quit = true,
        //                     event::KeyCode::Char('e') => self.input_mode = InputMode::Editing,
        //                     _ => (),
        //                 }
        //             }
        //
        //             InputMode::Editing => 
        //             {
        //                 match key.code
        //                 {
        //                     event::KeyCode::Enter => 
        //                     {
        //                         self.submit_message();
        //                         self.input_mode = InputMode::Normal;
        //                     }
        //
        //                     event::KeyCode::Char(to_insert) => 
        //                     {
        //                         self.enter_char(to_insert);
        //                     }
        //
        //                     event::KeyCode::Backspace => 
        //                     {
        //                         self.delete_char();
        //                     }
        //
        //                     event::KeyCode::Left => 
        //                     {
        //                         self.move_cursor_left();
        //                     }
        //
        //                     event::KeyCode::Right => 
        //                     {
        //                         self.move_cursor_right();
        //                     }
        //                     _ => ()
        //                 }
        //             }
        //         }
        //
        //     }
        // }
    }
}
