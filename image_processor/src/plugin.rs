use anyhow::{Result, anyhow};
use libloading::Library;
use std::{
    ffi::CString,
    path::{Path, PathBuf},
};

type ProcessImageFn = unsafe extern "C" fn(u32, u32, *mut u8, *const i8);

fn find_plugin_library(plugin_path: &Path, plugin_name: &str) -> Result<PathBuf> {
    let extension = if cfg!(target_os = "windows") {
        "dll"
    } else {
        "so"
    };

    let file_name = if cfg!(target_os = "windows") {
        format!("{}_plugin.{}", plugin_name, extension)
    } else {
        format!("lib{}_plugin.{}", plugin_name, extension)
    };

    let full_path = plugin_path.join(file_name);

    if full_path.exists() && full_path.is_file() {
        Ok(full_path)
    } else {
        Err(anyhow!(
            "Плагин '{}' не найден по пути {:?}. Ожидался файл {:?}",
            plugin_name,
            plugin_path,
            full_path
        ))
    }
}

pub fn run_plugin(
    plugin_path: &Path,
    plugin_name: &str,
    width: u32,
    height: u32,
    rgba: &mut [u8],
    params: &str,
) -> Result<()> {
    let check_len = (width as usize)
        .checked_mul(height as usize)
        .and_then(|pixels| pixels.checked_mul(4));

    match check_len {
        Some(v) => {
            if rgba.len() != v {
                return Err(anyhow!(
                    "Длина буфера не совпадает с рачетной: ожидалось {}, получено {}",
                    rgba.len(),
                    v
                ));
            }
        },
		None => return Err(anyhow!("Ошибка проверки длины буфера"))
    }

    let path = find_plugin_library(plugin_path, plugin_name)?;

    let lib = unsafe { Library::new(path)? };

    let c_params = CString::new(params)?;

    let fn_lib: libloading::Symbol<ProcessImageFn> = unsafe { lib.get(b"process_image")? };

    unsafe {
        fn_lib(width, height, rgba.as_mut_ptr(), c_params.as_ptr());
    };

    Ok(())
}
