
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};

const MAX_RUNTIME: Duration = Duration::from_secs(180 * 60);
const MOVE_INTERVAL: Duration = Duration::from_secs(15);
const STEP: i32 = 5;

fn logout() {
    println!("Logging out...");

    let _ = Command::new("gnome-session-quit")
        .args(["--logout", "--no-prompt"])
        .status();
}

fn move_mouse_smoothly(
    mouse: &sdl2::mouse::MouseUtil,
    window: &sdl2::video::Window,
    current_x: &mut i32,
    current_y: &mut i32,
    target_x: i32,
    target_y: i32,
) {
    while *current_x != target_x || *current_y != target_y {
        if *current_x < target_x {
            *current_x = (*current_x + STEP).min(target_x);
        } else if *current_x > target_x {
            *current_x = (*current_x - STEP).max(target_x);
        }

        if *current_y < target_y {
            *current_y = (*current_y + STEP).min(target_y);
        } else if *current_y > target_y {
            *current_y = (*current_y - STEP).max(target_y);
        }

        mouse.warp_mouse_in_window(window, *current_x, *current_y);

        thread::sleep(Duration::from_millis(10));
    }
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
    let mut current_x = 0;
    let mut current_y = 0;

    let start_time = Instant::now();
    let mut last_move = Instant::now();

    println!("Mouse Mover started.");
    println!("Maximum runtime: 180 minutes.");
    println!("Press Shift + 3 to exit and log out.");

    let should_logout;

    'running: loop {
        if start_time.elapsed() >= MAX_RUNTIME {
            println!("Timer expired.");
            should_logout = true;
            break 'running;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    should_logout = true;
                    break 'running;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    keymod,
                    ..
                } if keymod.intersects(
                    Mod::LSHIFTMOD | Mod::RSHIFTMOD
                ) => {
                    should_logout = true;
                    break 'running;
                }

                _ => {}
            }
        }

        if last_move.elapsed() >= MOVE_INTERVAL {
            let (target_x, target_y) = positions[i % positions.len()];

            println!(
                "Moving mouse to ({}, {})",
                target_x,
                target_y
            );

            move_mouse_smoothly(
                &mouse,
                &canvas.window(),
                &mut current_x,
                &mut current_y,
                target_x,
                target_y,
            );

            i += 1;
            last_move = Instant::now();
        }

        thread::sleep(Duration::from_millis(100));
    }

    drop(canvas);
    drop(event_pump);
    drop(sdl_context);

    
    //     logout();
}
