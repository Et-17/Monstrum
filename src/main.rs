mod random;

fn main() {
    let graphics_state_result = graphics::initialization::initialize(String::from("Test Window"));

    let mut graphics_state = match graphics_state_result {
        Err(e) => {
            println!("ERROR: {:}", e);
            std::process::exit(1)
        }
        Ok(gs) => gs,
    };

    loop {
        handle_frame_result(graphics::do_frame(&mut graphics_state));
    }
}

fn handle_frame_result(frame: graphics::FrameResult) {
    match frame {
        graphics::FrameResult::Normal => {}
        graphics::FrameResult::CloseProgram => std::process::exit(0),
        graphics::FrameResult::Exception => std::process::exit(1),
    }
}
