use std::time::Instant;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Archer", 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 1280, 720)
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        let now = Instant::now();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running Ok(()),
                _ => {}
            }
        }

        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();

        let elapsed = now.elapsed().as_secs_f64();
        let fps = 1.0 / elapsed;

        print!("\rFrame rendered in {elapsed} seconds. That's {fps} FPS!");
    }
}
