use anyhow::{Ok, Result};
use libloading::{Library, Symbol};
use std::{
    ffi::CString,
    path::{Path, PathBuf},
};

use crate::error::ImageProcessorError;

type ProcessImageFn = unsafe extern "C" fn(u32, u32, *mut u8, *const i8);

struct LoadedPlugin {
    _library: Library,
    process_image: ProcessImageFn,
}

fn find_plugin_library(plugin_path: &Path, plugin_name: &str) -> Result<PathBuf> {
    let file_name = if cfg!(target_os = "windows") {
        format!("{}_plugin.dll", plugin_name)
    } else if cfg!(target_os = "macos") {
        format!("lib{}_plugin.dylib", plugin_name)
    } else {
        format!("lib{}_plugin.so", plugin_name)
    };

    let full_path = plugin_path.join(file_name);

    if full_path.exists() && full_path.is_file() {
        Ok(full_path)
    } else {
        Err(ImageProcessorError::PluginLibraryNotFound(
            plugin_name.to_string(),
            plugin_path.to_str().unwrap().to_string(),
            full_path.to_str().unwrap().to_string(),
        )
        .into())
    }
}

/// Возвращает структуру LoadedPlugin с библиотекой и указатем на символ функции
/// # Safety
/// Связывает две сущности до конца их использования
unsafe fn load_plugin(plugin_path: &Path, plugin_name: &str) -> Result<LoadedPlugin> {
    let path = find_plugin_library(plugin_path, plugin_name)?;

    let lib = unsafe {
        Library::new(&path).map_err(|e| ImageProcessorError::PluginLibraryLoad {
            path: path.to_str().unwrap().to_string(),
            source: e,
        })
    }?;

    let fn_lib: Symbol<ProcessImageFn> = unsafe {
        lib.get(b"process_image")
            .map_err(|e| ImageProcessorError::PluginSymbolLoad {
                path: path.to_str().unwrap().to_string(),
                source: e,
            })
    }?;
    let func_ptr = *fn_lib;

    Ok(LoadedPlugin {
        _library: lib,
        process_image: func_ptr,
    })
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
                return Err(ImageProcessorError::InvalidRgbaBufferLen {
                    expected: rgba.len(),
                    actual: v,
                }
                .into());
            }
        }
        None => return Err(ImageProcessorError::InvalidRgbaBufferLenNone.into()),
    }

    let c_params = CString::new(params).map_err(ImageProcessorError::InvalidParamsCString)?;

    let lp = unsafe { load_plugin(plugin_path, plugin_name)? };

    unsafe {
        (lp.process_image)(width, height, rgba.as_mut_ptr(), c_params.as_ptr());
    };

    Ok(())
}
