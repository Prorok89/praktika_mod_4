use std::{ffi::CStr, slice::from_raw_parts_mut};

use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Default)]
struct Params {
	#[serde(default = "default_radius")]
	radius : usize,
	#[serde(default = "default_iterations")]
	iterations: usize
}

fn default_radius() -> usize {
    1
}

fn default_iterations() -> usize {
    1
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
    blur(width as usize, height as usize, data, current_params);
}

fn blur(width: usize, height: usize, rgba: &mut [u8], params: Params) {
    if width == 0 || height == 0 {
        return;
    }

    let radius = params.radius;
    let iterations = params.iterations.max(1);

    for _ in 0..iterations {
        let src = rgba.to_vec();

        for y in 0..height {
            for x in 0..width {
                let mut sums = [0u32; 4];
                let mut count = 0u32;

                let y_start = y.saturating_sub(radius);
                let y_end = (y + radius).min(height - 1);
                let x_start = x.saturating_sub(radius);
                let x_end = (x + radius).min(width - 1);

                for ny in y_start..=y_end {
                    for nx in x_start..=x_end {
                        let idx = offset(width, nx, ny);
                        sums[0] += src[idx] as u32;
                        sums[1] += src[idx + 1] as u32;
                        sums[2] += src[idx + 2] as u32;
                        sums[3] += src[idx + 3] as u32;
                        count += 1;
                    }
                }

                let dst = offset(width, x, y);
                rgba[dst] = (sums[0] / count) as u8;
                rgba[dst + 1] = (sums[1] / count) as u8;
                rgba[dst + 2] = (sums[2] / count) as u8;
                rgba[dst + 3] = (sums[3] / count) as u8;
            }
        }
    }
}

fn offset(width: usize, x: usize, y: usize) -> usize {
    (y * width + x) * 4
}


/*
radius
horizontal
*/