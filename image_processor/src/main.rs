use anyhow::{Result, anyhow};
use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

use clap::{Parser, ValueEnum};
use image::{ImageBuffer, Rgba};
#[derive(Parser, Debug)]
struct Cli {
    /// Путь к исходному PNG-изображению
    #[arg(short, long)]
    input: PathBuf,
    /// Путь, по которому будет сохранено обработанное изображение
    #[arg(short, long)]
    output: PathBuf,
    /// Имя плагина
    #[arg(short, long, value_enum)]
    plugin: PluginType,
    /// Путь к текстовому файлу с параметрами обработки
    #[arg(long)]
    params: String,
    /// Путь к директории, где находится плагин
    #[arg(long, default_value = "target/debug")]
    plugin_path: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum PluginType {
    Mirror,
    Blur,
}
/*
void process_image(
    uint32_t width,
    uint32_t height,
    uint8_t* rgba_data,
    const char* params
);
*/
fn main() -> Result<()> {
    let args = Cli::parse();

    println!("{:?}", args);

    let path_input = Path::new(&args.input);
    let path_output = Path::new(&args.output);
    let path_param = Path::new(&args.params);

    if path_input.exists()
        && path_input.is_file()
        && is_png(path_input)?
        && path_param.exists()
        && path_param.is_file()
    {
        let png = image::open(path_input)?;
        let png_rgba = png.to_rgba8();
        let (width, height) = png_rgba.dimensions();
        let mut buf = png_rgba.into_raw();
		let param = fs::read_to_string(path_param)?;

        match args.plugin {
            PluginType::Blur => {
                println!("выбран Blur");
				blur_plugin::process_image(width as usize, height as usize, &mut buf, &param);
            }
            PluginType::Mirror => {
                println!("Выбра Miror");
                mirror_plugin::process_image(width as usize, height as usize, &mut buf, &param);
            }
        }

        let out_image: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, buf)
                .ok_or(anyhow!("Ошибка при работе с буфером картинки",))?;

        out_image.save_with_format(path_output, image::ImageFormat::Png)?;
    } else {
        return Err(anyhow!(
            "Файл {:?} должен иметь расширение .png",
            path_input
        ));
    }

    Ok(())
}

fn is_png(path: &Path) -> Result<bool> {
    let mut file = File::open(path)?;

    let mut header = [0u8; 8];
    _ = file.read_exact(&mut header);

    let png_header = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    Ok(header == png_header)
}
/*

input
output
plugin - invert
params
plugin-path - target/debug
*/
