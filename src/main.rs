
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};

const MAX_RUNTIME: Duration = Duration::from_secs(3 * 60);

fn logout() {
    let _ = Command::new("gnome-session-quit")
        .args(["--logout", "--no-prompt"])
        .status();
}

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video = sdl_context.video().expect("Video initialization failed");

    let window = video
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

    let mouse = sdl_context.mouse();

    let positions = [(100, 100), (200, 200), (300, 300)];
    let mut i = 0;
    let mut last_move = Instant::now();

    // Watchdog timer starts here.
    let start_time = Instant::now();

    'running: loop {
        // Maximum runtime check.
        if start_time.elapsed() >= MAX_RUNTIME {
            println!("Timer expired. Logging out...");
            break 'running;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }

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

        if last_move.elapsed() >= Duration::from_secs(15) {
            let (x, y) = positions[i % positions.len()];

            mouse.warp_mouse_in_window(&canvas.window(), x, y);

            println!("Mouse moved to ({}, {})", x, y);

            i += 1;
            last_move = Instant::now();
        }

        thread::sleep(Duration::from_millis(100));
    }

    // Terminate the application and request logout.
    drop(canvas);
    drop(event_pump);
    drop(sdl_context);

    logout();
}
