use crate::engine::Engine;
use cgmath::Vector2;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas};

const INIT_SIZE: Vector2<u32> = Vector2::new(1024, 1024);
const BACKGROUND_COLOR: Color = Color::RGB(0x10, 0x10, 0x18);

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

    let mut middle_section = ui_square;
    middle_section.set_width(middle_section.width() / 2);
    middle_section.center_on(ui_square.center());

    let mut matrix = middle_section;
    matrix.set_width((matrix.width() as f32 * (7.0 / 8.0)) as u32);
    matrix.center_on(middle_section.center());

    canvas.set_draw_color(Color::WHITE);
    canvas.draw_rect(ui_square).unwrap();
    canvas.draw_rect(middle_section).unwrap();
    canvas.draw_rect(matrix).unwrap();
    canvas.present();
}
