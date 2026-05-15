use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct ForecastResponse {
    list: Vec<ForecastItem>,
    city: CityInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct ForecastItem {
    dt: u64,
    dt_txt: String,
    main: MainData,
    wind: WindData,
    weather: Vec<WeatherInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MainData {
    temp: f64,
    humidity: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct WindData {
    speed: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct WeatherInfo {
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CityInfo {
    name: String,
    country: String,
}

#[tauri::command]
async fn fetch_weather() -> Result<ForecastResponse, String> {
    dotenv().ok();

    let api_key = env::var("API_KEY").map_err(|_| "API_KEY missing".to_string())?;

    let latitude = 28.6139;
    let longitude = 77.2090;

    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&appid={}&units=metric",
        latitude, longitude, api_key
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let status = response.status();

    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();

        return Err(format!("OpenWeather API error {}: {}", status, body));
    }

    let weather_data = response
        .json::<ForecastResponse>()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))?;

    Ok(weather_data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_weather])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
