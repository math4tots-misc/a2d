extern crate a2d;
use a2d::Graphics2D;
use a2d::Instance;
use a2d::SpriteSheetDesc;
use futures::executor::block_on;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn main() {
    // simple_logger::init_with_level(log::Level::Debug).unwrap();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize {
            width: 1200,
            height: 800,
        })
        .build(&event_loop)
        .unwrap();

    let size = window.inner_size();
    let mut state = block_on(Graphics2D::new(size.width, size.height, &window)).unwrap();
    state.set_scale([1200.0 / 800.0, 1.0]);
    state
        .set_sheet(0, SpriteSheetDesc::Bytes(include_bytes!("happy-tree.png")))
        .unwrap();
    state.set_batch(0, 0).unwrap();
    let batch = state.get_batch_mut(0).unwrap();
    batch.add(
        Instance::builder()
            .src([0.0, 0.0, 0.75, 0.75])
            .dest([0.25, 0.25, 0.75, 0.75])
            .rotate(3.14 / 3.0)
            .build(),
    );
    batch.add(
        Instance::builder()
            .src([0.0, 0.0, 0.5, 0.5])
            .dest([0.0, 0.0, 0.25, 0.25])
            .rotate(0.0)
            .build(),
    );
    batch.add(
        Instance::builder()
            .src([0.75, 0.75, 1.0, 1.0])
            .dest([0.5, 0.5, 1.0, 1.0])
            .rotate(0.0)
            .build(),
    );
    batch.add(
        Instance::builder()
            .src([0.0, 0.75, 0.2, 1.0])
            .dest([0.5, 0.5, 1.0, 1.0])
            .rotate(0.0)
            .build(),
    );
    batch.add(
        Instance::builder()
            .src([0.0, 0.75, 0.2, 1.0])
            .dest([-0.1, 0.0, 0.1, 0.1])
            .rotate(0.0)
            .build(),
    );

    let start = std::time::SystemTime::now();

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(_) => {
            {
                let instance = state.get_batch_mut(0).unwrap().get_mut(0);
                let dur = start.elapsed().unwrap().as_secs_f32();
                instance.set_rotation((dur / 6.0).fract() * 2.0 * std::f32::consts::PI);
            }
            state.render();
            std::thread::yield_now();
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(code),
                    ..
                } => match code {
                    VirtualKeyCode::Up => {
                        let scale = state.scale();
                        let scale = [scale[0] * 2.0, scale[1] * 2.0];
                        state.set_scale(scale);
                    }
                    VirtualKeyCode::Down => {
                        let scale = state.scale();
                        let scale = [scale[0] / 2.0, scale[1] / 2.0];
                        state.set_scale(scale);
                    }
                    _ => {}
                },
                _ => {}
            },
            WindowEvent::Resized(physical_size) => {
                state.resized(physical_size.width, physical_size.height);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resized(new_inner_size.width, new_inner_size.height);
            }
            _ => {}
        },
        _ => {}
    })
}
