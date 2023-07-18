pub mod canvas;
mod web;

use canvas::Canvas;

#[tokio::main]
async fn main() {
	let r = 2048;
	let canvas = Canvas::new(r, r);

	let image = canvas.as_image();
	image.save("/tmp/out.jpg").unwrap();

	web::launch().await;
}
