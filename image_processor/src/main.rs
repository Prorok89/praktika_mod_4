use log::error;
use std::{fmt::Display, path::PathBuf, process::ExitCode};

use clap::{Parser, ValueEnum};

use image_processor::{ArgsParam, run};

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
    params: PathBuf,
    /// Путь к директории, где находится плагин
    #[arg(long, default_value = "target/debug")]
    plugin_path: PathBuf,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum PluginType {
    Mirror,
    Blur,
}

impl Display for PluginType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PluginType::Mirror => write!(f, "mirror"),
            PluginType::Blur => write!(f, "blur"),
        }
    }
}

fn main() -> ExitCode {
    env_logger::init();
	
    let args_cli = Cli::parse();

    let args = ArgsParam {
        input: args_cli.input,
        output: args_cli.output,
        plugin: args_cli.plugin.to_string(),
        params: args_cli.params,
        plugin_path: args_cli.plugin_path,
    };

    match run(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(er) => {
            error!("{}", er);
            ExitCode::FAILURE
        }
    }
}
