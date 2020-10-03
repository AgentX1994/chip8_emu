use chip8_emu::cpu::Cpu;
use chip8_emu::screen;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;

const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 320;

// The size of one screen pixel in the window
const SCREEN_PIXEL_WIDTH_ON_WINDOW: f32 = WINDOW_WIDTH as f32 / screen::WIDTH as f32;
const SCREEN_PIXEL_HEIGHT_ON_WINDOW: f32 = WINDOW_HEIGHT as f32 / screen::HEIGHT as f32;

fn print_usage(name: &str) {
    println!("Usage: {} <rom to load>", name);
}

fn draw(canvas: &mut WindowCanvas, pixels: &[u8]) {
    assert!(pixels.len() as u16 == screen::SIZE);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for y in 0..screen::HEIGHT {
        for x in 0..screen::WIDTH {
            let index = x + (y * screen::WIDTH);
            if pixels[index as usize] == 1 {
                // Calculate coordinates
                let window_x = (x as f32) * SCREEN_PIXEL_WIDTH_ON_WINDOW;
                let window_y = (y as f32) * SCREEN_PIXEL_HEIGHT_ON_WINDOW;
                // fill in pixel on screen
                canvas
                    .fill_rect(Rect::new(
                        window_x as i32,
                        window_y as i32,
                        SCREEN_PIXEL_WIDTH_ON_WINDOW as u32,
                        SCREEN_PIXEL_HEIGHT_ON_WINDOW as u32,
                    ))
                    .unwrap();
            }
        }
    }

    canvas.present();
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        let name = if args.len() == 0 {
            "chip8_emu"
        } else {
            &args[0][..]
        };
        print_usage(name);
        return;
    }
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Chip8 Emulator", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut cpu = Cpu::new();

    cpu.load_program(args[1].as_str())
        .expect("Could not load program");

    'running: loop {
        cpu.emulate_cycle();
        if cpu.draw_needed() {
            draw(&mut canvas, cpu.get_pixel_data());
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { .. } | Event::KeyUp { .. } => cpu.handle_input(event),
                _ => {}
            }
        }

        const CLOCK_SPEED: u32 = 500; // HZ
        const NANOS_PER_SECOND: u32 = 1_000_000_000;
        const SLEEP_TIME: u32 = NANOS_PER_SECOND / CLOCK_SPEED;
        ::std::thread::sleep(Duration::new(0, SLEEP_TIME));
    }
}
