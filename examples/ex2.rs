use a2d::Graphics2D;
use a2d::TextGridDim;
use futures::executor::block_on;
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let width = 800;
    let height = 600;
    let logical_size = LogicalSize { width, height };
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(logical_size)
        .build(&event_loop)
        .unwrap();

    let physical_size = PhysicalSize::from_logical(logical_size, window.scale_factor());

    let mut graphics = block_on(Graphics2D::new(
        physical_size.width,
        physical_size.height,
        &window,
    ))
    .unwrap();
    graphics.set_scale([logical_size.width as f32, logical_size.height as f32]);

    let TextGridDim { nrows, ncols } = graphics.init_text_grid(80).unwrap();

    for r in 0..nrows {
        for c in 0..ncols {
            graphics.draw_char(r, c, 'x').unwrap();
        }
    }

    graphics.draw_text(0, 0, "HeLlo WwOoRrLlDd!D").unwrap();
    graphics.flush().unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::RedrawRequested(_) => {
                graphics.force_render().unwrap();
                std::thread::yield_now();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id: _,
            } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::ReceivedCharacter(ch) => {
                    graphics.draw_char(5, 5, *ch).unwrap();
                    graphics.flush().unwrap();
                    window.request_redraw();
                }
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(code),
                        ..
                    } => match code {
                        VirtualKeyCode::Escape => {
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    },
                    _ => {}
                },
                WindowEvent::Resized(physical_size) => {
                    // TODO: update size
                    let logical_size =
                        LogicalSize::from_physical(*physical_size, window.scale_factor());
                    graphics.resized(physical_size.width, physical_size.height);
                    graphics.set_scale([logical_size.width, logical_size.height]);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // TODO: update scale factor and size
                    let physical_size = **new_inner_size;
                    let logical_size =
                        LogicalSize::from_physical(physical_size, window.scale_factor());
                    graphics.resized(physical_size.width, physical_size.height);
                    graphics.set_scale([logical_size.width, logical_size.height]);
                }
                _ => {}
            },
            _ => {}
        }
    });
}
