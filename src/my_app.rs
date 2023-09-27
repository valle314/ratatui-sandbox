use std::io;
use ratatui::{ backend::CrosstermBackend, Terminal };
use crossterm::event;

// my crates 
use crate::event_tui;
use crate::components::my_paragraph;


pub trait Renderable {
    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> bool;
}


pub struct App 
{
    should_quit: bool,
    component: my_paragraph::MyParagraph,
    comps: Vec<Box<dyn Renderable>>
}
    


impl App {
    pub fn new() -> Self
    {
        App {
            should_quit: false,
            component: my_paragraph::MyParagraph::new(),
            comps: vec![Box::new(my_paragraph::MyParagraph::new())] 
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

        for c in &mut self.comps
        {
            c.handle_key_event(key_event);
        }
        self.component.handle_key_event(key_event);

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
    }
}
