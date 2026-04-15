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
	horizontal : bool,
	#[serde(default)]
	vertical: bool
}

pub fn process_image(width: usize, height: usize, rgba_data: &mut [u8], params: &str) {
	let p = serde_json::from_str::<Params>(params).ok().unwrap();
	mirror(width, height, rgba_data, p);
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

/*
horizontal
vertical
*/