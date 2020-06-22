extern crate a2d;
use a2d::Graphics2D;
use a2d::Instance;
use a2d::SpriteBatch;
use a2d::SpriteSheet;
use futures::executor::block_on;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const DISPLAY_WIDTH: f32 = 1200.0;
const DISPLAY_HEIGHT: f32 = 800.0;
const WIDTH: f32 = DISPLAY_WIDTH / DISPLAY_HEIGHT;
const HEIGHT: f32 = 1.0;

pub fn main() {
    // simple_logger::init_with_level(log::Level::Debug).unwrap();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize {
            width: DISPLAY_WIDTH,
            height: DISPLAY_HEIGHT,
        })
        .build(&event_loop)
        .unwrap();

    let mut state = block_on(Graphics2D::from_winit_window(&window)).unwrap();
    state.set_scale([WIDTH, HEIGHT]);
    let sheet = SpriteSheet::from_color(&mut state, [1.0, 0.5, 0.5]).unwrap();
    let mut batch1 = SpriteBatch::new(sheet);
    batch1.add(Instance::new(
        [0.0, 0.0, 1.0, 1.0],
        [0.25, 0.25, 0.75, 0.75],
        3.14 / 3.0,
    ));
    batch1.add(Instance::new(
        [0.0, 0.0, 1.0, 1.0],
        [0.0, 0.0, 0.25, 0.25],
        0.0,
    ));
    batch1.add(Instance::new(
        [0.0, 0.0, 1.0, 1.0],
        [0.5, 0.5, 1.0, 1.0],
        0.0,
    ));
    batch1.add(Instance::new(
        [0.0, 0.0, 1.0, 1.0],
        [0.5, 0.5, 1.0, 1.0],
        0.0,
    ));
    batch1.add(Instance::new(
        [0.0, 0.0, 1.0, 1.0],
        [-0.1, 0.0, 0.1, 0.1],
        0.0,
    ));
    let mut batch2 =
        SpriteBatch::new(SpriteSheet::from_color(&mut state, [0.5, 0.5, 0.1]).unwrap());
    batch2.add(Instance::new(
        [0.0, 0.0, 1.0, 1.0],
        [0.5, 0.0, WIDTH, HEIGHT / 2.0],
        0.2,
    ));

    let start = std::time::SystemTime::now();

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(_) => {
            {
                let instance = batch1.get_mut(0);
                let dur = start.elapsed().unwrap().as_secs_f32();
                instance.set_rotation((dur / 6.0).fract() * 2.0 * std::f32::consts::PI);
            }
            state.render(&[&batch1, &batch2]);
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
                state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(**new_inner_size);
            }
            _ => {}
        },
        _ => {}
    })
}
