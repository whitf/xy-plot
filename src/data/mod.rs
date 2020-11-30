pub struct Point {
	pub label:			String,
	pub x:				i64,
	pub y:				i64,
}

impl Point {
	pub fn new(x: i64, y: i64, label: String) -> Self {
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
	pub x_min:				i64,
	pub x_max:				i64,
	pub y_min:				i64,
	pub y_max:				i64,
}

impl Data {
	pub fn new(label_x: String, label_y: String) -> Self {
		let data: Vec<Point> = Vec::new();

		let x_min = 0i64;
		let x_max = 25i64;
		let y_min = 0i64;
		let y_max = 25i64;

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

