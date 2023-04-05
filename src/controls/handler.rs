//! This manages the input system. First you need to run
//! InputHandler::initialize, which will create the sdl2 event pump, then call
//! handle_events() each step in order to handle the events that happened since
//! the last step.

use sdl2::event::Event;

/// This is the primary struct that gets passed around. It can contain whatever
/// is necessary for the input system being used. Because this module uses SDL2,
/// it contains the event pump.
pub struct InputHandler {
    pub event_pump: sdl2::EventPump,
}

impl InputHandler {
    /// Creates a new InputHandler
    fn new(event_pump: sdl2::EventPump) -> InputHandler {
        InputHandler { event_pump }
    }

    /// Creates a new event pump and initializes an InputHandler with it
    pub fn initialize(context: &mut sdl2::Sdl) -> Result<InputHandler, String> {
        Ok(Self::new(context.event_pump()?))
    }

    /// Handles the new events
    pub fn handle_events(&mut self) {
        self.event_pump
            .poll_iter()
            .for_each(|e| Self::handle_event(&e))
    }

    /// Handles a single event
    pub fn handle_event(event: &Event) {
        match event {
            Event::Quit { .. } => std::process::exit(0),
            _ => {}
        }
    }
}
