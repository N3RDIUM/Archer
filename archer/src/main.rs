use std::time::Duration;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;

use flume::unbounded;
use json::{object, JsonValue};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Just testing flume.
    let data: JsonValue = object! {
        foo: false,
        bar: null,
        answer: 42,
        list: [null, "world", true]
    };

    let (tx, rx) = unbounded::<JsonValue>();
    tx.send(data.clone()).unwrap();
    assert_eq!(rx.recv().unwrap(), data);

    let window = video_subsystem
        .window("Archer", 480, 360)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 480, 360)
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
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

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
