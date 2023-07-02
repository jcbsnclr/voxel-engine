mod renderer;
mod input;
mod world;

use winit::{
    window::{WindowBuilder, CursorGrabMode},
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent, DeviceEvent, KeyboardInput, VirtualKeyCode, ElementState},
    dpi::LogicalSize
};

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        // .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("creating window");
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Voxel Engine")
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(1280, 720))
        .build(&event_loop)
        .unwrap();

    window.focus_window();
    window.set_cursor_grab(CursorGrabMode::Locked)
        .expect("failed to grab cursor");

    let mut renderer = renderer::Renderer::init(window)?;
    let mut input = input::InputManager::new();

    log::info!("starting event loop");
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { window_id, ref event } if renderer.window().id() == window_id => {
                match event {
                    // handles user press escape or pressing the close button
                    WindowEvent::CloseRequested |
                    WindowEvent::KeyboardInput { input: KeyboardInput { 
                        virtual_keycode: Some(VirtualKeyCode::Escape), state: ElementState::Pressed, ..
                    }, .. } => *control_flow = ControlFlow::Exit,


                    event @ WindowEvent::KeyboardInput { .. } => input.process_keyboard(event),

                    _ => ()
                }
            },

            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => input.process_mouse(delta),

            Event::RedrawRequested(window_id) => if renderer.window().id() == window_id {
                let (dx, dy) = input.delta();
                renderer.camera.rotate(dx as f32 * 0.04, dy as f32 * 0.04);

                renderer.camera.travel(
                    input.is_pressed(VirtualKeyCode::W),
                    input.is_pressed(VirtualKeyCode::S),
                    input.is_pressed(VirtualKeyCode::A),
                    input.is_pressed(VirtualKeyCode::D),
                    input.is_pressed(VirtualKeyCode::Space),
                    input.is_pressed(VirtualKeyCode::C),
                );

                match renderer.render() {
                    Err(e) => log::error!("rendering failed: {}", e),
                    Ok(_) => ()
                }

                input.process_mouse((0.0, 0.0));
            },

            Event::MainEventsCleared => {
                renderer.window().request_redraw()
            },

            _ => ()
        }
    })
}
