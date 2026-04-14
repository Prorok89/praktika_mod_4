use clap::{Parser, ValueEnum};
#[derive(Parser, Debug)]
struct Cli {
	/// Путь к исходному PNG-изображению
	#[arg(short, long)]
	input : Option<String>,
	/// Путь, по которому будет сохранено обработанное изображение
	#[arg(short, long)]
	output : Option<String>,
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

fn main() {
	let args = Cli::parse();

	println!("{:?}", args);

	match args.plugin {
		PluginType::Blur => {
			println!("выбран Blur");
		},
		PluginType::Mirror => {
			println!("Выбра Miror")
		}
	}

    println!("Hello, world!");
}
/*

input
output
plugin - invert
params
plugin-path - target/debug
*/