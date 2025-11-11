use std::path::PathBuf;
use std::env;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = PathBuf::from(&manifest_dir);
    
    // Добавляем путь к библиотеке vosk для линковки
    // На Windows библиотека находится в корне проекта
    println!("cargo:rustc-link-search=native={}", manifest_path.display());
    
    // Для Linux нужно установить переменную окружения
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-env=LD_LIBRARY_PATH={}", manifest_path.display());
    }
}
