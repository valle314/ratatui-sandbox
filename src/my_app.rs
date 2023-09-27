use std::io;
use ratatui::{ backend::CrosstermBackend, Terminal };
use crossterm::event;

// my crates 
use crate::event_tui;
use crate::components::my_paragraph;


pub trait Component
{
    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> bool;
    fn render_app(&mut self, frame: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>);
    fn render_relative_app(&mut self, frame: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>, rect: ratatui::layout::Rect);
}


pub struct App 
{
    should_quit: bool,
    comps: Vec<Box<dyn Component>>
}
    
impl App {
    pub fn new() -> Self
    {
        App {
            should_quit: false,
            comps: vec![
                Box::new(my_paragraph::MyParagraph::new()),
                // Box::new(my_box::MyBox::new())
            ]
        }
    }


    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> ()
    {

        let events = event_tui::EventHandler::new(250);

        loop 
        {
            let _ = terminal.draw(|f| {
                for c in &mut self.comps
                {
                    c.render_app(f);
                }
            });

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
        let mut handled = false;
        for c in &mut self.comps
        {
            // TODO only handle key if focused!
            if !handled
            {
                handled = c.handle_key_event(key_event);
            }
            else { break; }
        }

        // these keybinds apply to the whole app
        if !handled 
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
        }

    }
}
