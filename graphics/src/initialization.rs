use crate::IOState;

pub fn initialize(title: String) -> Result<IOState, String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(title.as_str(), 0, 0)
        .fullscreen_desktop()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let event_pump = sdl_context.event_pump()?;

    Ok(IOState::new(canvas, event_pump))
}
