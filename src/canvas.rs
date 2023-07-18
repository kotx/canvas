use image::{ImageBuffer, Rgba};
use ndarray::{Array3, Axis, Zip};

use self::{
	channel::{A, B, G, R},
	ordering::{A_IDX, B_IDX, G_IDX, R_IDX},
};

pub(crate) mod ordering {
	pub const R_IDX: u8 = 0;
	pub const G_IDX: u8 = 1;
	pub const B_IDX: u8 = 2;
	pub const A_IDX: u8 = 3;
}

pub mod channel {
	use super::ordering::{A_IDX, B_IDX, G_IDX, R_IDX};

	pub(crate) const fn channel_to_flag(idx: u8) -> u8 {
		2u8.pow(idx as u32)
	}

	pub const R: u8 = channel_to_flag(R_IDX);
	pub const G: u8 = channel_to_flag(G_IDX);
	pub const B: u8 = channel_to_flag(B_IDX);
	pub const A: u8 = channel_to_flag(A_IDX);
}

#[derive(Debug)]
pub enum Error {
	OutOfBounds { width: bool, height: bool },
}

// https://stackoverflow.com/a/56762490/15441777
fn array_to_image(arr: &Array3<u8>) -> ImageBuffer<Rgba<u8>, &[u8]> {
	assert!(arr.is_standard_layout());

	let (height, width, _) = arr.dim();
	let raw = arr.as_slice_memory_order().unwrap();

	ImageBuffer::from_raw(width as u32, height as u32, raw)
		.expect("container should have the right size for the image dimensions")
}

#[derive(Clone)]
pub struct Canvas {
	pixels: Array3<u8>,
}

impl Canvas {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			pixels: Array3::zeros((height, width, 4)),
		}
	}

	pub fn set(&mut self, x: usize, y: usize, channels: u8, value: u8) -> Result<(), Error> {
		let (height, width, _) = self.pixels.dim();
		if x >= width || y >= height {
			Err(Error::OutOfBounds {
				width: x >= width,
				height: y >= height,
			})
		} else {
			if channels & R == R {
				self.pixels[[y, x, R_IDX as usize]] = value;
			}

			if channels & G == G {
				self.pixels[[y, x, G_IDX as usize]] = value;
			}

			if channels & B == B {
				self.pixels[[y, x, B_IDX as usize]] = value;
			}

			if channels & A == A {
				self.pixels[[y, x, A_IDX as usize]] = value;
			}

			Ok(())
		}
	}

	pub fn set_rgb(&mut self, x: usize, y: usize, value: [u8; 3]) -> Result<(), Error> {
		self.set(x, y, R, value[0])?;
		self.set(x, y, G, value[1])?;
		self.set(x, y, B, value[2])?;
		self.set(x, y, A, 255)
	}

	pub fn as_image(&self) -> ImageBuffer<Rgba<u8>, &[u8]> {
		array_to_image(&self.pixels)
	}

	pub fn merge(&mut self, other: &Canvas) {
		Zip::from(self.pixels.lanes_mut(Axis(2)))
			.and(other.pixels.lanes(Axis(2)))
			.for_each(|mut self_pixels, other_pixels| {
				if other_pixels[A_IDX as usize] != 0 {
					self_pixels.assign(&other_pixels);
				}
			});
	}
}
