use std::{env, fs::File, io::Read, path::{Path, PathBuf}};
use anyhow::{Result, anyhow};

use clap::{Parser, ValueEnum};
use image::GenericImageView;
#[derive(Parser, Debug)]
struct Cli {
	/// Путь к исходному PNG-изображению
	#[arg(short, long)]
	input : PathBuf,
	/// Путь, по которому будет сохранено обработанное изображение
	#[arg(short, long)]
	output : PathBuf,
	/// Имя плагина
	#[arg(short, long, value_enum)]
	plugin : PluginType,
	/// Путь к текстовому файлу с параметрами обработки
	#[arg(long)]
	params : Option<String>,
	/// Путь к директории, где находится плагин
	#[arg(long, default_value= "target/debug")]
	plugin_path :  Option<String>
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
fn main() -> Result<()>{
	let args = Cli::parse();

	println!("{:?}", args);

	let path_input = Path::new(&args.input);
	let path_output = Path::new(&args.output);
	
	if is_png(path_input)? && path_output.is_dir() {

		let png = image::open(path_input)?;
		let png_rgba = png.to_rgba8();
		let (width, height) = png_rgba.dimensions();
		let buf = png_rgba.into_raw();

		match args.plugin {
			PluginType::Blur => {
				println!("выбран Blur");
			},
			PluginType::Mirror => {
				println!("Выбра Miror")
			}
		}
	}
	else
	{
		return Err(anyhow!("Файл {:?} должен иметь расширение .png", path_input));
	}
    
    Ok(())
}

fn is_png(path: &Path) -> Result<bool>
{
	let mut file = File::open(path)?;

	let mut header = [0u8;8];
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