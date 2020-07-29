use a2d::Color;
use a2d::Graphics2D;
use futures::executor::block_on;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let width = 800;
    let height = 600;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize { width, height })
        .build(&event_loop)
        .unwrap();

    let mut graphics = block_on(Graphics2D::new(width, height, &window)).unwrap();

    graphics.set_pixel(0, 0, [0.0, 1.0, 1.0]).unwrap();
    for x in 0..width {
        for y in 0..height {
            graphics
                .set_pixel(
                    x as usize,
                    y as usize,
                    [1.0, x as f32 / width as f32, y as f32 / height as f32],
                )
                .unwrap();
        }
    }

    let mut x = 0;
    let mut y = 0;
    let mut ncycles = 0;
    let mut color: Color = [0.0, 0.5, 1.0].into();

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(_) => {
            for _ in 0..100 {
                x += 1;
                if x >= width {
                    x = 0;
                    y += 1;
                }
                if y >= height {
                    y = 0;
                    x = 0;
                    println!("Finished cycle");
                    ncycles += 1;
                    color = [
                        (ncycles % 2) as f32 * 1.0,
                        (ncycles % 3) as f32 * 0.2,
                        (ncycles % 2 - 1) as f32 * 1.0,
                    ]
                    .into();
                }
                graphics.set_pixel(x as usize, y as usize, color).unwrap();
            }
            graphics.render().unwrap();
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
                graphics.resized(logical_size.width, logical_size.height);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // TODO: update scale factor and size
                let logical_size =
                    LogicalSize::from_physical(**new_inner_size, window.scale_factor());
                graphics.resized(logical_size.width, logical_size.height);
            }
            _ => {}
        },
        _ => {}
    });
}
