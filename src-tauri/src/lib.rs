use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WeatherResponse {
    current: CurrentWeather,
    hourly: HourlyWeather,
    daily: DailyWeather,
}

#[derive(Serialize, Deserialize)]
struct CurrentWeather {
    temperature_2m: f64,
    relative_humidity_2m: f64,
    apparent_temperature: f64,
    is_day: u8,
    percipitation: f64,
    weather_code: u64,
    wind_speed_10m: f64,
}

#[derive(Serialize, Deserialize)]
struct HourlyWeather {
    time: Vec<String>,
    temperature: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
struct DailyWeather {
    time: Vec<String>,
    temperature_2m_max: Vec<f64>,
    temperature_2m_min: Vec<f64>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
