use crate::IOState;
use sdl2::event::Event;

pub enum FrameResult {
    Normal,
    CloseProgram,
    Exception,
}

pub fn do_frame(state: &mut IOState) -> FrameResult {
    for event in state.event_pump.poll_iter() {
        match handle_event(&event) {
            Some(f) => return f,
            None => {}
        }
    }

    return FrameResult::Normal;
}

pub fn handle_event(event: &Event) -> Option<FrameResult> {
    match event {
        Event::Quit { .. } => return Some(FrameResult::CloseProgram),
        _ => None,
    }
}
