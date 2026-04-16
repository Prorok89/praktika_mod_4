pub mod error;
pub mod plugin_loader;

use anyhow::{Ok, Result};
use image::{ImageBuffer, Rgba};
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use crate::error::ImageProcessorError;
use crate::plugin_loader::run_plugin;

pub struct ArgsParam {
    pub input: PathBuf,
    pub output: PathBuf,
    pub plugin: String,
    pub params: PathBuf,
    pub plugin_path: PathBuf,
}

pub fn is_png(path: &PathBuf) -> Result<bool> {
    let mut file = File::open(&path).map_err(ImageProcessorError::InputFileNotFound)?;

    let mut header = [0u8; 8];
    _ = file.read_exact(&mut header);

    let png_header = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    Ok(header == png_header)
}

pub fn run(args: ArgsParam) -> Result<()> {
    if !is_png(&args.input)? {
        return Err(ImageProcessorError::InputFileNotPNG.into());
    }

    _ = File::open(&args.input).map_err(ImageProcessorError::InputFileNotFound)?;
    _ = File::open(&args.params).map_err(ImageProcessorError::ParamsFileNotFound)?;

    if !args.input.is_file() {
        return Err(ImageProcessorError::InputIsNotFile.into());
    }

    if !args.params.is_file() {
        return Err(ImageProcessorError::ParamsIsNotFile.into());
    }

    let png = image::open(&args.input).map_err(ImageProcessorError::Image)?;
    let png_rgba = png.to_rgba8();
    let (width, height) = png_rgba.dimensions();
    let mut buf = png_rgba.into_raw();
    let param =
        fs::read_to_string(&args.params).map_err(ImageProcessorError::ParamsFileNotFound)?;

    run_plugin(
        &args.plugin_path,
        &args.plugin.to_string(),
        width,
        height,
        &mut buf,
        &param,
    )?;

    let out_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, buf)
        .ok_or(ImageProcessorError::ImageBufferFromRawFailed)?;
    
    out_image.save_with_format(&args.output, image::ImageFormat::Png).map_err(ImageProcessorError::Image)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_is_png_positive() {
        let mut file = NamedTempFile::new().unwrap();
        let png_header = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        file.write_all(&png_header).unwrap();

        assert!(is_png(&file.path().to_path_buf()).unwrap());
    }

    #[test]
    fn test_is_png_negative() {
        let mut file = NamedTempFile::new().unwrap();
        let not_png_header = [0, 0, 0, 0, 0, 0, 0, 0];
        file.write_all(&not_png_header).unwrap();

        assert!(!is_png(&file.path().to_path_buf()).unwrap());
    }

    #[test]
    fn test_is_png_not_found() {
        let path = PathBuf::from("non_existent_file.png");
        let result = is_png(&path);
        assert!(result.is_err());
    }
}