use lazy_static::lazy_static;
use peak_alloc::PeakAlloc;
use systemstat::{Platform, System};
use std::thread;
use std::time::Duration;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

lazy_static! {
    static ref SYS: System = System::new();
}

#[tauri::command]
pub fn get_current_ram_usage() -> String {
    let bytes = PEAK_ALLOC.current_usage();
    format!("{:.2}", bytes as f64 / (1024.0 * 1024.0))
}

#[tauri::command]
pub fn get_peak_ram_usage() -> String {
    let bytes = PEAK_ALLOC.peak_usage();
    format!("{:.2}", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
}

#[tauri::command]
pub fn get_cpu_temp() -> String {
    if let Ok(cpu_temp) = SYS.cpu_temp() {
        String::from(format!("{}", cpu_temp))
    } else {
        String::from("error")
    }
}

// https://github.com/valpackett/systemstat/blob/trunk/examples/info.rs
#[tauri::command(async)]
pub async fn get_cpu_usage() -> String {
    if let Ok(cpu) = SYS.cpu_load_aggregate() {
        thread::sleep(Duration::from_secs(1));
        let cpu = cpu.done().unwrap();
        String::from(format!("{}", cpu.user * 100.0))
    } else {
        String::from("error")
    }
}
