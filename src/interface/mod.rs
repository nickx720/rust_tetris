use crate::engine::{Engine, Matrix};
use cgmath::{ElementWise, EuclideanSpace, Vector2};
use render::ScreenColor;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas};
use sub_rect::SubRect;

use self::sub_rect::Align;

mod render;
mod sub_rect;

const INIT_SIZE: Vector2<u32> = Vector2::new(1024, 1024);
const BACKGROUND_COLOR: Color = Color::RGB(0x10, 0x10, 0x18);
const PLACEHOLDER_1: Color = Color::RGB(0x66, 0x77, 0x77);
const PLACEHOLDER_2: Color = Color::RGB(0x66, 0x77, 0x77);

struct Tick;
struct LockdownTick;

pub fn run(engine: Engine) {
    let sdl = sdl2::init().expect("Failed to initialise SDL2");

    let event_subsytem = sdl.event().expect("Failed to acquire event subsystem");
    event_subsytem.register_custom_event::<Tick>().unwrap();
    event_subsytem
        .register_custom_event::<LockdownTick>()
        .unwrap();
    let mut canvas = {
        let video = sdl.video().expect("Failed to acquire display");

        let window = video
            .window("Tetris", INIT_SIZE.x, INIT_SIZE.y)
            .position_centered()
            .resizable()
            .build()
            .expect("Failed to create window");
        window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .expect(" Failed to get render canvas")
    };

    let mut events = sdl.event_pump().expect("Failed to get event loop");

    event_subsytem.push_custom_event(Tick).unwrap();
    event_subsytem.push_custom_event(LockdownTick).unwrap();

    loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => return,
                Event::User { .. } if event.as_user_event_type::<Tick>().is_some() => {
                    println!("Found tick event")
                }
                Event::User { .. } if event.as_user_event_type::<LockdownTick>().is_some() => {
                    println!("Found lockdown event")
                }
                _ => {}
            }
        }

        draw(&mut canvas, &engine);
    }
}

fn draw(canvas: &mut Canvas<sdl2::video::Window>, engine: &Engine) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    let viewport = canvas.viewport();
    let ui_square = SubRect::absolute(viewport, (1.0, 1.0), None);

    let matrix = ui_square
        .sub_rect((0.5, 1.0), None)
        .sub_rect((7.0 / 8.0, 7.0 / 8.0), None);

    let up_next = ui_square
        .sub_rect((0.25, 0.25), Some((Align::Far, Align::Near)))
        .sub_rect((0.75, 0.75), None);

    let hold = ui_square
        .sub_rect((0.25, 0.25), Some((Align::Near, Align::Near)))
        .sub_rect((0.75, 0.75), None);

    let queue = ui_square
        .sub_rect((0.25, 0.75), Some((Align::Far, Align::Far)))
        .sub_rect((5.0 / 8.0, 23.0 / 24.0), Some((Align::Center, Align::Near)));

    let score = ui_square
        .sub_rect((0.25, 11.0 / 16.0), Some((Align::Near, Align::Far)))
        .sub_rect((7.0 / 8.0, 8.0 / 11.0), Some((Align::Center, Align::Near)));

    // Draw graphics
    canvas.set_draw_color(PLACEHOLDER_1);

    for subrect in [&matrix, &up_next, &hold, &queue, &score] {
        canvas.fill_rect(Rect::from(subrect)).unwrap();
    }

    let matrix_origin = matrix.bottom_left();
    let matrix_dims = matrix.size();

    let matrix_cells = Vector2::new(Matrix::WIDTH, Matrix::HEIGHT)
        .cast::<u32>()
        .unwrap();
    for (coord, cell) in engine.cells() {
        let cell_color = match cell {
            Some(cell) => cell,
            None => continue,
        };
        let coord = coord.to_vec().cast::<u32>().unwrap();
        let this = (coord + Vector2::new(0, 1))
            .mul_element_wise(matrix_dims)
            .div_element_wise(matrix_cells);
        let next = (coord + Vector2::new(1, 0))
            .mul_element_wise(matrix_dims)
            .div_element_wise(matrix_cells);

        let cell_rect = Rect::new(
            matrix_origin.x + this.x as i32,
            matrix_origin.y - this.y as i32,
            next.x - this.x,
            this.y - next.y,
        );
        canvas.set_draw_color(cell_color.screen_color());
        canvas.fill_rect(cell_rect).unwrap();
    }

    canvas.present();
}
