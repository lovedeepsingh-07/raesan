#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    env_logger::Builder::new()
        .filter_module("raesan", log::LevelFilter::Debug)
        .filter_module("raesan_native", log::LevelFilter::Debug)
        .filter_module("raesan_native_lib", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    raesan_native_lib::run()
}
