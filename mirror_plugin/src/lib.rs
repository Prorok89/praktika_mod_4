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

    let lenght: usize = match (width as usize)
        .checked_mul(height as usize)
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
		from_raw_parts_mut(rgba_data, lenght)
	};
    mirror(width as usize, height as usize, data, current_params);
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