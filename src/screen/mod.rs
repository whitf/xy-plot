use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
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
		let title = format!("xy-plot v.{}", VERSION);
		let fps = 60u64;
		let tic = Duration::from_millis(1000u64 / fps);

		let video_subsystem = sdl_context.sdl_context.video()
			.expect("Failed to init video_subsystem from sdl_context");
		let window = video_subsystem.window(&title, width, height)
			.position_centered()
			.build()
			.unwrap();
		let canvas = window.into_canvas()
			.accelerated()
			.present_vsync()
			.build()
			.expect("Failed to create canvas from window");

		Screen {
			canvas,
			sdl_context,
			tic,
		}
	}

	fn label(&mut self, _data: &data::Data) -> Result<(), String> {
		let width = self.canvas.viewport().w;
		let height = self.canvas.viewport().h;

		let origin = Point::new(50, height - 50);
		let x_end = Point::new(width - 50, height - 50);
		let y_end  = Point::new(50, 50);

		let old_draw_color = self.canvas.draw_color();
		self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));

		self.canvas.draw_line(origin, x_end)?;
		self.canvas.draw_line(Point::new(x_end.x - 10, height - 50 - 10), x_end)?;
		self.canvas.draw_line(Point::new(x_end.x - 10, height - 50 + 10), x_end)?;

		let interval: i32 = (x_end.x - origin.x) / 4;
		for i in 1..4 {
			let a = Point::new(i * interval, origin.y - 5);
			let b = Point::new(i * interval, origin.y + 5);
			self.canvas.draw_line(a, b)?;
		}

		self.canvas.draw_line(origin, y_end)?;
		self.canvas.draw_line(Point::new(y_end.x - 10, y_end.y + 10), y_end)?;
		self.canvas.draw_line(Point::new(y_end.x + 10, y_end.y + 10), y_end)?;

		let interval: i32 = (origin.y - y_end.y) / 4;
		for i in 1..4 {
			let a = Point::new(origin.x - 5, origin.y - (i * interval));
			let b = Point::new(origin.x + 5, origin.y - (i * interval));
			self.canvas.draw_line(a, b)?;
		}

		self.canvas.set_draw_color(old_draw_color);

		Ok(())
	}

	fn process_event(event: Event) -> bool {
		match event {
			Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
				return false;
			},
			_ => {},
		}

		true
	}

	fn plot(&mut self, data: &data::Data) -> Result<(), String> {
		let width = self.canvas.viewport().w;
		let height = self.canvas.viewport().h;

		let x_scale: i32 = width / data.x_max;
		let y_scale: i32 = height / data.y_max;

		let origin = Point::new(50, height - 50);

		let old_draw_color = self.canvas.draw_color();

		self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));

		for p in &data.data {
			let center = Screen::translate(Point::new(p.x, p.y), origin, x_scale, y_scale);
			self.canvas.fill_rect(Rect::new(center.x - 5, center.y - 5, 10, 10))?;
		}

		self.canvas.set_draw_color(old_draw_color);
		
		Ok(())
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

			self.label(&data).unwrap();

			self.plot(&data).unwrap();
			
			self.canvas.present();

			thread::sleep(self.tic);
		}
	}

	fn translate(p: Point, origin: Point, x_scale: i32, y_scale: i32) -> Point {
		let x = origin.x + (p.x * x_scale);
		let y = origin.y - (p.y * y_scale); 

		Point::new(x, y)
	}
}
