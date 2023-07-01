mod renderer;

use winit::{
    window::WindowBuilder,
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent},
    dpi::LogicalSize
};

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("creating window");
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Voxel Engine")
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(1280, 720))
        .build(&event_loop)
        .unwrap();

    let mut renderer = renderer::Renderer::init(window)?;

    log::info!("starting event loop");
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { window_id, ref event } if renderer.window().id() == window_id => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    _ => ()
                }
            },

            Event::RedrawRequested(window_id) => if renderer.window().id() == window_id {
                match renderer.render() {
                    Err(e) => log::error!("rendering failed: {}", e),
                    Ok(_) => ()
                }
            },

            Event::MainEventsCleared => renderer.window().request_redraw(),

            _ => ()
        }
    })
}
