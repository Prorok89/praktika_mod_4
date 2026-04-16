use std::{ffi::CStr, slice::from_raw_parts_mut};

use serde::Deserialize;

/*
void process_image(
	uint32_t width,
	uint32_t height,
	uint8_t* rgba_data,
	const char* params
);

*/
#[derive(Debug, Clone, Copy, Deserialize, Default)]
struct Params {
	#[serde(default)]
	horizontal: bool,
	#[serde(default)]
	vertical: bool,
}

#[unsafe(no_mangle)]
pub extern "C" fn process_image(
	width: *const u32,
	height: *const u32,
	rgba_data: *mut u8,
	params: *const i8,
) {
	if rgba_data.is_null() {
		return;
	}

	let data_width = unsafe { *width };
	let data_height = unsafe { *height };

	let length: usize = match (data_width as usize)
		.checked_mul(data_height as usize)
		.and_then(|f| f.checked_mul(4))
	{
		Some(v) => v,
		None => return,
	};



	let mut current_params = Params::default();

	if !params.is_null() {
		let str = unsafe {
			CStr::from_ptr(params)
		};
		let str = match str.to_str() {
			Ok(v) => v,
			Err(_) => return
		};

		if !str.trim().is_empty() {

			if let Ok(v) = serde_json::from_str::<Params>(str) {
				current_params = v;
			}
		}
	}

	let data = unsafe {
		from_raw_parts_mut(rgba_data, length)
	};
	mirror(data_width as usize, data_height as usize, data, current_params);
}

fn mirror(width: usize, height: usize, rgba: &mut [u8], params: Params) {
	if width == 0 || height == 0 || (!params.horizontal && !params.vertical) {
		return;
	}

	let src = rgba.to_vec();
	for y in 0..height {
		for x in 0..width {
			let src_x = if params.horizontal { width - 1 - x } else { x };
			let src_y = if params.vertical { height - 1 - y } else { y };
			let dst_idx = offset(width, x, y);
			let src_idx = offset(width, src_x, src_y);
			rgba[dst_idx..dst_idx + 4].copy_from_slice(&src[src_idx..src_idx + 4]);
		}
	}
}

fn offset(width: usize, x: usize, y: usize) -> usize {
	(y * width + x) * 4
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_offset() {
		assert_eq!(offset(10, 0, 0), 0);
		assert_eq!(offset(10, 1, 0), 4);
		assert_eq!(offset(10, 0, 1), 40);
		assert_eq!(offset(10, 1, 1), 44);
	}

	#[test]
	fn test_mirror_horizontal() {
		let width = 2;
		let height = 1;
		let mut data = vec![
			255, 0, 0, 255, // Red
			0, 255, 0, 255, // Green
		];
		let params = Params { horizontal: true, vertical: false };
		mirror(width, height, &mut data, params);

		let expected = vec![
			0, 255, 0, 255, // Green
			255, 0, 0, 255, // Red
		];
		assert_eq!(data, expected);
	}

	#[test]
	fn test_mirror_vertical() {
		let width = 1;
		let height = 2;
		let mut data = vec![
			255, 0, 0, 255, // Red
			0, 255, 0, 255, // Green
		];
		let params = Params { horizontal: false, vertical: true };
		mirror(width, height, &mut data, params);

		let expected = vec![
			0, 255, 0, 255, // Green
			255, 0, 0, 255, // Red
		];
		assert_eq!(data, expected);
	}

	#[test]
	fn test_mirror_both() {
		let width = 2;
		let height = 2;
		let mut data = vec![
			255, 0, 0, 255, // (0,0) Red
			0, 255, 0, 255, // (1,0) Green
			0, 0, 255, 255, // (0,1) Blue
			255, 255, 0, 255, // (1,1) Yellow
		];
		let params = Params { horizontal: true, vertical: true };
		mirror(width, height, &mut data, params);

		let expected = vec![
			255, 255, 0, 255, // (0,0) <- (1,1)
			0, 0, 255, 255,   // (1,0) <- (0,1)
			0, 255, 0, 255,   // (0,1) <- (1,0)
			255, 0, 0, 255,   // (1,1) <- (0,0)
		];
		assert_eq!(data, expected);
	}

	#[test]
	fn test_mirror_no_op() {
		let width = 2;
		let height = 1;
		let mut data = vec![
			255, 0, 0, 255,
			0, 255, 0, 255,
		];
		let original = data.clone();
		let params = Params { horizontal: false, vertical: false };
		mirror(width, height, &mut data, params);
		assert_eq!(data, original);
	}

	#[test]
	fn test_process_image_basic() {
		let width: u32 = 2;
		let height: u32 = 1;
		let mut rgba_data = vec![
			255, 0, 0, 255, // Red
			0, 255, 0, 255, // Green
		];
		let params_json = r#"{"horizontal": true}"#;
		let params_ptr = std::ffi::CString::new(params_json).unwrap().into_raw();

		unsafe {
			process_image(&width, &height, rgba_data.as_mut_ptr(), params_ptr);
			let _ = std::ffi::CString::from_raw(params_ptr); // clean up
		}

		let expected = vec![
			0, 255, 0, 255, // Green
			255, 0, 0, 255, // Red
		];
		assert_eq!(rgba_data, expected);
	}

	#[test]
	fn test_process_image_null_data() {
		let width: u32 = 2;
		let height: u32 = 1;
		let params_json = r#"{"horizontal": true}"#;
		let params_ptr = std::ffi::CString::new(params_json).unwrap().into_raw();

		unsafe {
			process_image(&width, &height, std::ptr::null_mut(), params_ptr);
			let _ = std::ffi::CString::from_raw(params_ptr);
		}
	}

	#[test]
	fn test_process_image_invalid_params() {
		let width: u32 = 2;
		let height: u32 = 1;
		let mut rgba_data = vec![
			255, 0, 0, 255,
			0, 255, 0, 255,
		];
		let original = rgba_data.clone();
		let params_json = r#"{"invalid": "json"}"#; // Valid JSON, but not Params
		let params_ptr = std::ffi::CString::new(params_json).unwrap().into_raw();

		unsafe {
			process_image(&width, &height, rgba_data.as_mut_ptr(), params_ptr);
			let _ = std::ffi::CString::from_raw(params_ptr);
		}
		
		assert_eq!(rgba_data, original);
	}
}

