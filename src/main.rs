
use std::process::Command;
use std::time::{Duration, Instant};
use std::thread;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};

fn logout() {
    let _ = Command::new("gnome-session-quit")
        .args(["--logout", "--no-prompt"])
        .status();
}

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Video initialization failed");

    let window = video_subsystem
        .window("Mouse Mover", 2020, 1180)
        .position_centered()
        .fullscreen_desktop()
        .build()
        .expect("Window creation failed");

    let canvas = window.into_canvas()
        .build()
        .expect("Canvas creation failed");

    let mut event_pump = sdl_context.event_pump()
        .expect("Event pump creation failed");

    let mouse_util = sdl_context.mouse();

    let positions = [(100, 100), (200, 200), (300, 300)];
    let mut i = 0;
    let mut last_move = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    keymod,
                    ..
                } if keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD) => {
                    break 'running;
                }

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

    // Runs whenever the event loop exits normally.
    logout();
}
