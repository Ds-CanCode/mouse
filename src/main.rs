
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::Cursor;

use x11::{xlib, xtest};

const MAX_RUNTIME: Duration = Duration::from_secs(120 * 60);
const MOVE_INTERVAL: Duration = Duration::from_secs(2);
const SCROLL_INTERVAL: Duration = Duration::from_millis(500);
const STEP: i32 = 5;

// Create a custom 16x16 black arrow cursor.
fn create_black_cursor() -> Cursor {
    let arrow = [
        "#...............",
        "##..............",
        "#.#.............",
        "#..#............",
        "#...#...........",
        "#....#..........",
        "#.....#.........",
        "#......#........",
        "#.......#.......",
        "#....#####......",
        "#..##..#.........",
        "#.#...#..........",
        "##....#..........",
        "......#..........",
        "......#..........",
        ".......#.........",
    ];

    let mut bitmap = vec![0u8; 32];

    for (y, row) in arrow.iter().enumerate() {
        for (x, pixel) in row.chars().enumerate() {
            if pixel == '#' {
                let index = y * 2 + x / 8;
                bitmap[index] |= 1 << (7 - (x % 8));
            }
        }
    }

    // Same bitmap for data and mask:
    // opaque black pixels, transparent background.
    Cursor::new(
        &bitmap,
        &bitmap,
        16,
        16,
        0,
        0,
    )
    .expect("Cannot create black cursor")
}

fn logout() {
    println!("Logging out...");

    let _ = Command::new("gnome-session-quit")
        .args(["--logout", "--no-prompt"])
        .status();
}

// Generate system-wide mouse scroll events using XTest.
fn scroll_mouse(display: *mut xlib::Display) {
    unsafe {
        // Button 4 = scroll up.
        xtest::XTestFakeButtonEvent(display, 4, 1, 0);
        xtest::XTestFakeButtonEvent(display, 4, 0, 0);

        xlib::XFlush(display);
    }
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

        mouse.warp_mouse_in_window(
            window,
            *current_x,
            *current_y,
        );

        thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    // Initialize X11 connection.
    let display = unsafe {
        xlib::XOpenDisplay(std::ptr::null())
    };

    if display.is_null() {
        panic!("Cannot connect to X11 display");
    }

    let sdl_context =
        sdl2::init().expect("SDL initialization failed");

    let video = sdl_context
        .video()
        .expect("Video initialization failed");

    let window = video
        .window("Mouse Mover", 2020, 1180)
        .position_centered()
        .fullscreen_desktop()
        .build()
        .expect("Window creation failed");

    let canvas = window
        .into_canvas()
        .build()
        .expect("Canvas creation failed");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Event pump creation failed");

    let mouse = sdl_context.mouse();

    // Create and activate the custom black cursor.
    let black_cursor = create_black_cursor();
    black_cursor.set();

    let positions = [
        (100, 100),
        (200, 200),
        (300, 300),
        (400, 400),
        (500, 500),
        (600, 600),
        (700, 700),
        (800, 800),
        (900, 900),
        (1000, 1000),
        (1100, 1100),
        (1200, 1200),
        (1300, 1300),
        (1400, 1400),
        (1500, 1500),
    ];

    let mut i = 0;
    let mut current_x = 0;
    let mut current_y = 0;

    let start_time = Instant::now();
    let mut last_move = Instant::now();
    let mut last_scroll = Instant::now();

    println!("Mouse Mover started.");
    println!("Cursor: Black arrow");
    println!("Movement: Every 2 seconds");
    println!("Scroll: Down every 500 milliseconds");
    println!("Step: 5 pixels");
    println!("Maximum runtime: 180 minutes");
    println!("Press Shift + 3 to logout.");

    'running: loop {
        // 180-minute watchdog.
        if start_time.elapsed() >= MAX_RUNTIME {
            println!("Timer expired.");
            break 'running;
        }

        // Handle keyboard and window events.
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    keymod,
                    ..
                } if keymod.intersects(
                    Mod::LSHIFTMOD | Mod::RSHIFTMOD
                ) => {
                    println!("Shift + 3 pressed.");
                    break 'running;
                }

                _ => {}
            }
        }

        // Scroll mouse every 500 milliseconds.
        if last_scroll.elapsed() >= SCROLL_INTERVAL {
            scroll_mouse(display);
            last_scroll = Instant::now();
        }

        // Move mouse every 2 seconds.
        if last_move.elapsed() >= MOVE_INTERVAL {
            let (target_x, target_y) =
                positions[i % positions.len()];

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

    // Clean up SDL resources.
    drop(black_cursor);
    drop(canvas);
    drop(event_pump);
    drop(sdl_context);

    // Close X11 connection.
    unsafe {
        xlib::XCloseDisplay(display);
    }

    // Logout after normal application exit.
    logout();
}
