pub struct Point {
	pub label:			String,
	pub x:				i32,
	pub y:				i32,
}

impl Point {
	pub fn new(x: i32, y: i32, label: String) -> Self {
		Point {
			label,
			x,
			y,
		}
	}
}

pub struct Data {
	pub label_x:			String,
	pub label_y:			String,
	pub data:				Vec<Point>,
	pub x_min:				i32,
	pub x_max:				i32,
	pub y_min:				i32,
	pub y_max:				i32,
}

impl Data {
	pub fn new(label_x: String, label_y: String) -> Self {
		let data: Vec<Point> = Vec::new();

		let x_min = 0i32;
		let x_max = 25i32;
		let y_min = 0i32;
		let y_max = 25i32;

		Data {
			label_x,
			label_y,
			data,
			x_min,
			x_max,
			y_min,
			y_max,
		}
	}
}

