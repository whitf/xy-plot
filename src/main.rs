use clap::{App, Arg};

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
		.get_matches();

	let width = 1200u32;
	let height = 900u32;

	let mut sdl_context = SdlContext::init();
	let mut screen = screen::Screen::new(width, height, &mut sdl_context);
	let mut window = Window::new(&mut screen);

	//let mut data = data::Data::new("x".to_string(), "y".to_string());
	let data = data::Data::new("x".to_string(), "y".to_string());
	window.launch(data);
}
