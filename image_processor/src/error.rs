use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageProcessorError {
    #[error("Входной файл не PNG")]
    InputFileNotPNG,
    #[error("Входной путь указывает не на файл")]
    InputIsNotFile,
    #[error("Параметры указывает не на файл")]
    ParamsIsNotFile,
    #[error("Входной файл не найден: {0}")]
    InputFileNotFound(io::Error),
    #[error("Файл параметров не найден: {0}")]
    ParamsFileNotFound(io::Error),
    #[error("Плагин {0} не найден по пути {1}. Ожидался файл {2}")]
    PluginLibraryNotFound(String, String, String),
    #[error("Не удалось загрузить библиотеку плагина {path}: {source}")]
    PluginLibraryLoad {
        path: String,
        source: libloading::Error,
    },
    #[error("Не удалось найти функцию process_image в библиотеке {path}: {source}")]
    PluginSymbolLoad {
        path: String,
        source: libloading::Error,
    },
    #[error("Длина буфера не совпадает с рачетной: ожидалось {expected}, получено {actual}")]
    InvalidRgbaBufferLen { expected: usize, actual: usize },
    #[error("Параметры содержат нулевой байт и не могут быть переданы через C-строку")]
    InvalidParamsCString(#[from] std::ffi::NulError),
    #[error("Ошибка ввода/вывода: {0}")]
    Image(#[from] image::ImageError),
    #[error("Не удалось собрать изображение из RGBA-буфера")]
    ImageBufferFromRawFailed,
    #[error("Ошибка проверки длины буфера")]
    InvalidRgbaBufferLenNone,
}
