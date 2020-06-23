extern crate a2d;
use a2d::Graphics2D;
use a2d::TextGrid;
use futures::executor::block_on;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const DEFAULT_DISPLAY_WIDTH: f32 = 1200.0;
const DEFAULT_DISPLAY_HEIGHT: f32 = 800.0;
const WIDTH: f32 = DEFAULT_DISPLAY_WIDTH / DEFAULT_DISPLAY_HEIGHT;
const HEIGHT: f32 = 1.0;

/// Text example
pub fn main() {
    // simple_logger::init_with_level(log::Level::Debug).unwrap();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize {
            width: DEFAULT_DISPLAY_WIDTH,
            height: DEFAULT_DISPLAY_HEIGHT,
        })
        .build(&event_loop)
        .unwrap();

    let mut state = block_on(Graphics2D::from_winit_window(&window)).unwrap();
    state.set_scale([WIDTH, HEIGHT]);
    let char_sheet = TextGrid::courier_sprite_sheet(&mut state).unwrap();
    let mut tgrid = TextGrid::new(char_sheet, WIDTH / 40.0, [15, 10]);

    tgrid.write(0, 0, "Hello world!");

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(_) => {
            state.render(&[tgrid.batch()]);
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
            WindowEvent::ReceivedCharacter(ch) => {
                println!("char = {}", ch);
            }
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
