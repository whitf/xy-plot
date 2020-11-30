use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::thread;
use std::time::Duration;

use crate::SdlContext;
use crate::data;

pub struct Screen<'a> {
	canvas:					WindowCanvas,
	sdl_context:			&'a crate::SdlContext,
	tic:					Duration,
}

impl Screen<'_> {
	pub fn new<'a>(width: u32, height: u32, sdl_context: &'a mut SdlContext) -> Screen<'a> {
		const VERSION: &'static str = env!("CARGO_PKG_VERSION");

		let fps = 60u64;
		let tic = Duration::from_millis(1000u64 / fps);
		let bg_fill = Color::RGBA(0, 0, 0, 255);

		let video_subsystem = sdl_context.sdl_context.video()
			.expect("Failed to init video_subsystem from sdl_context");
		let window = video_subsystem.window(VERSION, width, height)
			.position_centered()
			.build()
			.unwrap();
		let mut canvas = window.into_canvas()
			.accelerated()
			.present_vsync()
			.build()
			.expect("Failed to create canvas from window");

		canvas.set_draw_color(bg_fill);

		Screen {
			canvas,
			sdl_context,
			tic,
		}
	}

	pub fn process_event(event: Event) -> bool {
		match event {
			Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
				return false;
			},
			_ => {},
		}

		true
	}

	pub fn run(&mut self, data: data::Data) {
		let context = &self.sdl_context.sdl_context;
		let mut event_pump = context.event_pump().unwrap();

		'plotloop: loop {
			for event in event_pump.poll_iter() {
				if !Screen::process_event(event) {
					break 'plotloop;
				}
			}

			self.canvas.clear();

			self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));

			self.canvas.fill_rect(Rect::new(0, 0, 1200, 900))
				.expect("failed to draw plotting rectangle");

			// draw graph labels

			// draw data

			self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
			self.canvas.present();

			thread::sleep(self.tic);
		}
	}
}