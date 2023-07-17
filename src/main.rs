#![feature(async_closure)]

pub mod canvas;
use std::time::Instant;

use canvas::Canvas;

fn main() {
	let mut canvas = Canvas::new(1024, 1024);
	let mut canvas_two = Canvas::new(1024, 1024);

	canvas.set_rgb(0, 0, [255, 255, 255]).unwrap();
	canvas_two.set_rgb(0, 1, [255, 255, 255]).unwrap();

	let now = Instant::now();
	canvas_two.merge(&canvas).unwrap();
	println!("{:?}", now.elapsed());

	let image = canvas_two.as_image();
	image.save("/tmp/out.png").unwrap();
}
