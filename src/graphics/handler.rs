//! This manages the graphics system. First you need to run
//! GraphicsHandler::initialize, which will create the window, then call
//! do_frame() in order to draw the next frame.

/// This is the primary struct that gets passed around. It can contain whatever
/// is necessary for the graphics system being used. Because this module uses SDL2,
/// it contains the canvas.
pub struct GraphicsHandler {
    pub canvas: sdl2::render::WindowCanvas,
}

impl GraphicsHandler {
    /// Creates a new GraphicsHandler struct
    fn new(canvas: sdl2::render::WindowCanvas) -> GraphicsHandler {
        GraphicsHandler { canvas }
    }

    /// Creates a new fullscreen_desktop window with the title given
    pub fn initialize(
        title: String,
        sdl_context: &mut sdl2::Sdl,
    ) -> Result<GraphicsHandler, String> {
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(title.as_str(), 0, 0)
            .fullscreen_desktop()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(GraphicsHandler::new(canvas))
    }

    /// Draws the next frame
    pub fn do_frame(&mut self) {
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }
}
