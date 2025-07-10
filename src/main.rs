use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Mouse Mover", 2020, 1180)
        .position_centered()
        .fullscreen_desktop()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mouse_util = sdl_context.mouse();

    let positions = [(100, 100), (200, 200), (300, 300)];
    let mut i = 0;
    let mut last_move = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        if last_move.elapsed() >= Duration::from_secs(60) {
            let (x, y) = positions[i % positions.len()];
            mouse_util.warp_mouse_in_window(&canvas.window(), x, y);
            println!("Mouse moved to ({}, {})", x, y);
            i += 1;
            last_move = Instant::now();
        }

        thread::sleep(Duration::from_millis(100));
    }
}
