#![allow(unused_imports)]
extern crate a2d;
use a2d::Graphics2D;
use a2d::Instance;
use a2d::SpriteBatch;
use a2d::SpriteSheet;
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
const DEFAULT_WIDTH: f32 = DEFAULT_DISPLAY_WIDTH / DEFAULT_DISPLAY_HEIGHT;
const DEFAULT_HEIGHT: f32 = 1.0;

fn normalized_dims(pixel_dims: [f32; 2]) -> [f32; 2] {
    let [width, height] = pixel_dims;
    [width / DEFAULT_DISPLAY_WIDTH * DEFAULT_WIDTH, height / DEFAULT_DISPLAY_HEIGHT * DEFAULT_HEIGHT]
}

struct State {
    start_time: std::time::Instant,
    cursor_batch: SpriteBatch,
    text_grid: TextGrid,
    coord: [u32; 2],
}

impl State {
    fn new(graphics: &mut Graphics2D) -> Self {
        let text_grid = graphics.new_text_grid(DEFAULT_WIDTH / 80.0, [160, 80]).unwrap();

        let mut cursor_batch = SpriteBatch::new(SpriteSheet::from_color(graphics, [1.0, 1.0, 1.0]).unwrap());
        cursor_batch.add(
            Instance::builder()
                .src([0.0, 0.0, 1.0, 1.0])
                .dest(text_grid.rect_for_coord([0, 0]))
                .rotate(0.0),
        );

        let coord = [0, 0];

        let mut state = Self {
            start_time: std::time::Instant::now(),
            cursor_batch,
            text_grid,
            coord,
        };

        state.set_cursor(coord);

        state
    }

    fn update(&mut self) {
        if self.start_time.elapsed().as_secs_f32().fract() > 0.5 {
            self.cursor_batch.get_mut(0).set_color_factor([0.0, 0.0, 0.0]);
        } else {
            self.cursor_batch.get_mut(0).set_color_factor([1.0, 1.0, 1.0]);
        }
    }

    fn batches(&self) -> Vec<&SpriteBatch> {
        vec![
            &self.cursor_batch,
            self.text_grid.batch(),
        ]
    }

    fn set_cursor(&mut self, row_col: [u32; 2]) {
        let rect = self.text_grid.rect_for_coord(row_col);
        self.cursor_batch.get_mut(0).set_dest(rect);
    }

    fn put_ch(&mut self, ch: char) {
        let [row, col] = self.coord;
        self.text_grid.write_ch([row, col], ch);
        self.incr();
    }

    fn incr(&mut self) {
        let [mut row, mut col] = self.coord;
        let [max_row, max_col] = self.text_grid.dimensions();
        col += 1;
        if col >= max_col {
            col = 0;
            row += 1;
        }
        if row >= max_row {
            col = 0;
            row = 0;
        }
        self.coord = [row, col];
        self.set_cursor(self.coord);
    }
}

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

    let mut graphics = block_on(Graphics2D::from_winit_window(&window)).unwrap();
    graphics.set_scale(normalized_dims([DEFAULT_DISPLAY_WIDTH, DEFAULT_DISPLAY_HEIGHT]));
    let mut state = State::new(&mut graphics);

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(_) => {
            graphics.render(&state.batches());
            std::thread::yield_now();
        }
        Event::MainEventsCleared => {
            state.update();
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
                        let scale = graphics.scale();
                        let scale = [scale[0] * 2.0, scale[1] * 2.0];
                        graphics.set_scale(scale);
                    }
                    VirtualKeyCode::Down => {
                        let scale = graphics.scale();
                        let scale = [scale[0] / 2.0, scale[1] / 2.0];
                        graphics.set_scale(scale);
                    }
                    _ => {}
                },
                _ => {}
            },
            WindowEvent::ReceivedCharacter(ch) => {
                println!("char = {}", ch);
                state.put_ch(*ch);
            }
            WindowEvent::Resized(physical_size) => {
                let width = physical_size.width;
                let height = physical_size.height;
                let dim = normalized_dims([width as f32, height as f32]);
                graphics.resize(*physical_size);
                graphics.set_scale(dim);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                let width = new_inner_size.width;
                let height = new_inner_size.height;
                let dim = normalized_dims([width as f32, height as f32]);
                graphics.resize(**new_inner_size);
                graphics.set_scale(dim);
            }
            _ => {}
        },
        _ => {}
    })
}
