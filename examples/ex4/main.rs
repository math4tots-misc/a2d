extern crate a2d;
use a2d::Graphics2D;
use a2d::Instance;
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

    let size = window.inner_size();
    let mut state = block_on(Graphics2D::new(size.width, size.height, &window)).unwrap();
    state.set_scale([WIDTH, HEIGHT]);

    state.set_sheet(0, [1.0, 0.5, 0.5]).unwrap();

    // // creating large numbers of SpriteBatches is insanely slow
    //
    // let mut batches = Vec::new();
    // const SIZE: usize = 100;
    // for r in 0..SIZE {
    //     for c in 0..SIZE {
    //         let mut batch = SpriteBatch::new(sheet.clone());
    //         batch.set_translation([c as f32 / 100.0, r as f32 / 100.0]);
    //         batch.add(Instance::new(
    //             [0.0, 0.0, 1.0, 1.0],
    //             [0.0, 0.0, 1.0 / SIZE as f32 / 2.0, 1.0 / SIZE as f32 / 2.0],
    //             3.14 / 2.0,
    //         ));
    //         batches.push(batch);
    //     }
    // }

    const SIZE: usize = 100;
    state.set_batch(0, 0).unwrap();
    let batch = state.get_batch_mut(0).unwrap();
    let len = 1.0 / SIZE as f32 / 2.0;
    for r in 0..SIZE {
        let oy = r as f32 * (1.0 / SIZE as f32);
        for c in 0..SIZE {
            let ox = c as f32 * (1.0 / SIZE as f32);
            if is_prime(r + c) {
                batch.add(
                    Instance::builder()
                        .src([0.0, 0.0, 1.0, 1.0])
                        .dest([ox, oy, ox + len, oy + len])
                        .rotate(3.14 / 2.0),
                );
            }
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
                state
                    .get_batch_mut(0)
                    .unwrap()
                    .set_translation([WIDTH * rot, 0.0]);
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

fn is_prime(x: usize) -> bool {
    let mut i = 2;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
