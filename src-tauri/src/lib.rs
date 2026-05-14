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

#[tauri::command]
async fn fetch_weather() -> Result<WeatherResponse, String> {
    let latitude = 28.6139;
    let longitude = -77.2090;

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,is_day,precipitation,weather_code,wind_speed_10m&hourly=temperature_2m&daily=temperature_2m_max,temperature_2m_min&timezone=Asia%2FSingapore",
        latitude,
        longitude
    );

    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;

    let weather_data = response
        .json::<WeatherResponse>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(weather_data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_weather])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
