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

/// Lots of sprite batches with lots of translation
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

    // // creating large numbers of SpriteBatches is insanely slow
    //
    // let mut batches = Vec::new();
    // const SIZE: usize = 100;
    // for r in 0..SIZE {
    //     for c in 0..SIZE {
    //         let mut batch = SpriteBatch::new(sheet.clone());
    //         batch.set_translation(Some([c as f32 / 100.0, r as f32 / 100.0]));
    //         batch.add(Instance::new(
    //             [0.0, 0.0, 1.0, 1.0],
    //             [0.0, 0.0, 1.0 / SIZE as f32 / 2.0, 1.0 / SIZE as f32 / 2.0],
    //             3.14 / 2.0,
    //         ));
    //         batches.push(batch);
    //     }
    // }

    const SIZE: usize = 100;
    let mut batch = SpriteBatch::new(sheet.clone());
    let len = 1.0 / SIZE as f32 / 2.0;
    for r in 0..SIZE {
        let oy = r as f32 * (1.0 / SIZE as f32);
        for c in 0..SIZE {
            let ox = c as f32 * (1.0 / SIZE as f32);
            batch.add(Instance::new(
                [0.0, 0.0, 1.0, 1.0],
                [ox, oy, ox + len, oy + len],
                3.14 / 2.0,
            ));
        }
    }

    let start = std::time::SystemTime::now();

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(_) => {
            // {
            //     let instance = batch1.get_mut(0);
            //     let dur = start.elapsed().unwrap().as_secs_f32();
            //     instance.set_rotation((dur / 6.0).fract() * 2.0 * std::f32::consts::PI);
            // }

            // let batch_refs: Vec<_> = batches.iter().collect();
            // state.render(&batch_refs);

            {
                let dur = start.elapsed().unwrap().as_secs_f32();
                let rot = (dur / 12.0).fract();
                batch.set_translation(Some([WIDTH * rot, 0.0]));
            }

            state.render(&[&batch]);

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
