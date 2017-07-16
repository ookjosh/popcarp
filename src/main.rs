extern crate sdl2;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};

use std::time::Duration;

fn main() {

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("Popcarp", 800, 600)
		.position_centered() // Move to center of screen
		.opengl() // Using opengl
		.build()
		.unwrap();

	let mut canvas = window.into_canvas()
		.present_vsync()
		.build().unwrap();

	// Create a texture creator
	let creator = canvas.texture_creator();
	let mut texture = creator
		.create_texture_target(PixelFormatEnum::RGBA8888, 400, 300)
		.unwrap();


	//canvas.set_draw_color(Color::RGB(255, 0, 0));
	//canvas.clear(); // Clear to last set color.
	//canvas.present(); // Show update

	let mut event_pump = sdl_context.event_pump().unwrap();

	let mut angle = 0.0; // For example rotation.

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} | Event::KeyDown {keycode : Some(Keycode::Escape), .. } => {
					break 'running
				},
				_ => {}
			}
		}

		angle = (angle + 0.5) % 360.0;

		// I think this is a 'closure' (|texture_canvas|)...
		// 
		canvas.with_texture_canvas(&mut texture, |texture_canvas| {
			texture_canvas.clear();
			texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
			texture_canvas.fill_rect(Rect::new(0,0,400,300)).unwrap();
		}).unwrap();
		canvas.set_draw_color(Color::RGBA(0,0,0,255));
		let dest = Some(Rect::new(0,0,400,300));
		canvas.clear();
		canvas.
			copy_ex(&texture, // Src texture
				None, // Source Rectangle or None for whole texture
				dest, // Destination rectangle or None for whole target.
				angle, // Rotation to be applied to dest
				Some(Point::new(400,300)), // Point to rotate around
				false, // Flip (I think hflip, vflip)
				false)
			.unwrap();
		canvas.present();

		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}

    println!("Hello, world!");
}

