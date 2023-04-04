pub struct IOState {
    pub canvas: sdl2::render::WindowCanvas,
    pub event_pump: sdl2::EventPump,
}

impl IOState {
    pub fn new(canvas: sdl2::render::WindowCanvas, event_pump: sdl2::EventPump) -> IOState {
        IOState { canvas, event_pump }
    }
}
