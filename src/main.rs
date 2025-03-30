extern crate sdl2;

// har hört att man ska använda dessa
// use sdl2::render::Canvas;
// use sdl2::video::Window;

use sdl2::pixels::Color as sdlColor;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
// use sdl2::render::{Canvas, RenderTarget};

use std::time::Duration;
use std::time::Instant;

struct Player {
    x: i128,
    y: i128,
    health: i16,
    speed: f16,
}

fn update(canvas: &mut Canvas<Window>) {
    return
}

fn draw(canvas: &mut Canvas<Window>) {
    return
}

fn main() {
    let width: i16 = 1500;
    let height: i16 = 800;
    let mut player = Player {
        x: 0,
        y: 0,
        health: 18_000,
        speed: 1.0,
    };


    let sdl_context = sdl2::init()
        .unwrap();

    let video_subsystem = sdl_context
        .video()
        .unwrap();

    let mut event_pump = sdl_context
        .event_pump()
        .unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();

    'running: loop
    {
        canvas.set_draw_color((0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::Quit {..} |
                Event::KeyDown{ keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown{ keycode: Some(Keycode::W), .. } => { /**/ }, 
                Event::KeyDown{ keycode: Some(Keycode::A), .. } => { /**/ }, 
                Event::KeyDown{ keycode: Some(Keycode::S), .. } => { /**/ }, 
                Event::KeyDown{ keycode: Some(Keycode::D), .. } => { /**/ }, 
                _ => {}
            }
        }

        update(&canvas);
        draw(&canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));
    }
}
