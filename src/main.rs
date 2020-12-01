use clap::{App, Arg};
use std::fs;
use toml::Value;

pub mod data;
pub mod screen;

pub struct SdlContext {
	pub _image_context:				sdl2::image::Sdl2ImageContext,
	pub sdl_context:				sdl2::Sdl,
	pub _ttf_context:				sdl2::ttf::Sdl2TtfContext,
}

impl SdlContext {
	fn init() -> SdlContext {
		SdlContext {
			_image_context: sdl2::image::init(sdl2::image::InitFlag::PNG)
				.expect("Init failed: image_context"),
			sdl_context: sdl2::init()
				.expect("Init failed: sdl_context"),
			_ttf_context: sdl2::ttf::init()
				.expect("Init failed: ttf_context"),
		}
	}
}

struct Window<'sdl> {
	screen: &'sdl mut screen::Screen<'sdl>,
}

impl Window<'_> {
	fn new<'sdl>(screen: &'sdl mut screen::Screen<'sdl>) -> Window<'sdl> {
		Window {
			screen,
		}
	}

	fn launch(&mut self, data: data::Data) {
		self.screen.run(data);
	}
}


fn main() {
	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	let matches = App::new("xy-plot")
		.version(VERSION)
		.about("A simple xy plotting utility.")
		.arg(Arg::with_name("file")
			.short("f")
			.long("file")
			.takes_value(true)
			.help("Specify a file to use for input data."))
		.arg(Arg::with_name("label-x")
			.short("x")
			.long("label-x")
			.takes_value(true)
			.help("Specify a label for the x-axis."))
		.arg(Arg::with_name("label-y")
			.short("y")
			.long("label-y")
			.takes_value(true)
			.help("Specify a label for the y-axis."))
		.get_matches();

	let width = 1200u32;
	let height = 900u32;

	let mut sdl_context = SdlContext::init();
	let mut screen = screen::Screen::new(width, height, &mut sdl_context);
	let mut window = Window::new(&mut screen);

	let data_file = matches.value_of("file").unwrap_or("data/data.toml");
	let mut data = data::Data::new("x".to_string(), "y".to_string());

	let toml_content = fs::read_to_string(&data_file).expect("Could not read toml data file.");
	let raw_data: Value = toml::from_str(&toml_content).expect("Could not parse data.");
	let mapped_data: &toml::map::Map<String, Value> = raw_data["xy"].as_table().unwrap();

	for (k, v) in mapped_data.iter() {
		match k.as_str() {
			"data" => {
				for d in v.as_array().unwrap() {
					let x = d["x"].as_integer().unwrap() as i32;
					let y = d["y"].as_integer().unwrap() as i32;

					let p = data::Point::new(x, y, "DataLabel".to_string());
					data.data.push(p);
				}

			},
			_ => {
				println!("Unrecognized key value in data file.");
			}
		}
	}

	println!("launching plotting window with {} points", data.data.len());


	window.launch(data);
}
