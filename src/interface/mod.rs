use crate::engine::Engine;
use cgmath::Vector2;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas};

const INIT_SIZE: Vector2<u32> = Vector2::new(1024, 1024);
const BACKGROUND_COLOR: Color = Color::RGB(0x10, 0x10, 0x18);
const MATRIX_COLOR: Color = Color::RGB(0x66, 0x77, 0x77);

pub fn run(_engine: Engine) {
    let sdl = sdl2::init().expect("Failed to initialise SDL2");

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

    loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => return,
                _ => {}
            }
        }

        draw(&mut canvas);
    }
}

fn draw(canvas: &mut Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    // Draw graphics
    let ui_square = {
        let Vector2 { x, y } = Vector2::from(canvas.viewport().size())
            .cast::<i32>()
            .unwrap();
        if x > y {
            let margin = (x / 2) - (y / 2);
            Rect::new(margin, 0, y as u32, y as u32)
        } else {
            let margin = (y / 2) - (x / 2);
            Rect::new(0, margin, x as u32, x as u32)
        }
    };

    let matrix = {
        let mut middle_section = ui_square;
        middle_section.set_width(middle_section.width() / 2);
        middle_section.center_on(ui_square.center());

        let mut matrix = middle_section;
        matrix.resize(
            (matrix.width() as f32 * (7.0 / 8.0)) as _,
            (matrix.height() as f32 * (7.0 / 8.0)) as _,
        );
        matrix.center_on(middle_section.center());
        matrix
    };

    let up_next = {
        let mut bounding_box = ui_square;
        let quarter = ui_square.width() / 4;
        bounding_box.resize(quarter, quarter);
        bounding_box.offset(3 * quarter as i32, 0);

        let mut rect = bounding_box;
        let inner_dim = bounding_box.width() * 3 / 4;
        rect.resize(inner_dim, inner_dim);
        rect.center_on(bounding_box.center());
        rect
    };

    let hold = {
        let mut bounding_box = ui_square;
        let quarter = ui_square.width() / 4;
        bounding_box.resize(quarter, quarter);

        let mut rect = bounding_box;
        let inner_dim = bounding_box.width() * 3 / 4;
        rect.resize(inner_dim, inner_dim);
        rect.center_on(bounding_box.center());
        rect
    };

    let queue = {
        let mut bounding_box = ui_square;
        let quarter = ui_square.width() / 4;
        bounding_box.resize(quarter, 3 * quarter);
        bounding_box.offset(3 * quarter as i32, quarter as _);

        let mut rect = bounding_box;
        let inner_width = bounding_box.width() * 5 / 8;
        let inner_height = bounding_box.height() * 23 / 24;
        rect.resize(inner_width, inner_height);
        rect.center_on(bounding_box.center());
        rect.set_y(bounding_box.top());
        rect
    };

    let score = {
        let mut bounding_box = ui_square;
        let quarter = ui_square.width() / 4;
        let sixteenth = quarter / 4;
        bounding_box.resize(quarter, 2 * quarter);
        bounding_box.offset(0, 5 * sixteenth as i32);

        let mut rect = bounding_box;
        let inner_width = bounding_box.width() * 7 / 8;
        rect.set_width(inner_width);
        rect.center_on(bounding_box.center());
        rect.set_y(bounding_box.top());
        rect
    };

    canvas.set_draw_color(MATRIX_COLOR);
    canvas.fill_rect(matrix).unwrap();
    canvas.fill_rect(up_next).unwrap();
    canvas.fill_rect(hold).unwrap();
    canvas.fill_rect(queue).unwrap();
    canvas.fill_rect(score).unwrap();
    canvas.present();
}
