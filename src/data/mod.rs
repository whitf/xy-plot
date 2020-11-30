pub struct Point {
	pub label:			String,
	pub x:				usize,
	pub y:				usize,
}

impl Point {
	pub fn new(label: String, x: usize, y: usize) -> Self {
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
}

impl Data {
	pub fn new(label_x: String, label_y: String) -> Self {
		let data: Vec<Point> = Vec::new();

		Data {
			label_x,
			label_y,
			data,
		}
	}
}

