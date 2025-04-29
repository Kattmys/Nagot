extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color as sdlColor;
// use sdl2::rect::Rect;
// use sdl2::render::{Canvas, RenderTarget};

use std::time::Duration;
// use std::time::Instant;

struct Player {
    x: i32,
    y: i32,
    health: i16,
    speed: f64,
}

fn generate_map() {
    let mut map = Vec::new();

    // map.push(vec![0, 0, 0, 0, 0, 0])
    // map.push(vec![0, 0, 1, 1, 0, 0])
    // map.push(vec![0, 1, 1, 1, 1, 0])
    // map.push(vec![0, 1, 1, 1, 1, 1])
    // map.push(vec![0, 0, 0, 1, 1, 1])

    return map
}

fn update() {
}

struct Coord(u32, u32);

fn draw(canvas: &mut Canvas<sdl2::video::Window>, player: Player) {
    return

    // let dimensions = Coord(1500, 800);
    // let block_size: u32 = 50;
    //
    // let start_x = player.x - (dimension.0 / 2);
    // let start_y = player.y - (dimension.1 / 2);
    //
    // let block_x = start_x / 50;
    // let block_y = start_y / 50;
    /* x_side is the x value relative to the 
       world, we later use it in x_side_2 to 
       determine which block we are trying to
       render and then get the right borders 
       x-value of that block by rounding 
       downwards and then adding 1 and 
       multiplying with 50. */
    // let x_side: u32 = (player.x - (dimension.0 / 2)) / block_size;
    // let x_side_2: u32 = (x_side + 1) * block_size;
    //
    // let y_side: u32 = (player.y - (dimension.1 / 2)) / block_size;
    // let y_side_2: u32 = (y_side + 1) * block_size;
    //
    // for x in x_side..x_side + dimensions.0 / block_size {
    //     for y in y_side..y_side + dimensions.1 / block_size {
    //         grid[x][y]
    //     }
    // }
}

fn main() {
    let dimensions = Coord(1500, 800);
    let width:  u32 = 1500;
    let height: u32 = 800;
    let mut player = Player {
        x: 0,
        y: 0,
        health: 18_000,
        speed: 1.0,
    };

    let map = Vec::new();


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


        update(&mut player);
        draw(&mut canvas, player);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));
    }
}
