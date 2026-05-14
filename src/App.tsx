import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";  
import "./App.css";
import {
  Cloud,
  CloudRain,
  Sun,
  Wind,
  Droplets,
  Thermometer,
} from "lucide-react";

type WeatherData = {
  current: {
    temperature_2m: number,
    relative_humidity_2m: number,
    apparent_temperature: number,
    is_day: number,
    percipitation: number,
    weather_code: number,
    wind_speed_10m: number,
  },
  hourly: {
    time: string[],
    temperature: number[],
  },
  daily: {
    time: string[],
    temperature_2m_max: number[],
    temperature_2m_min: number[],
  }
}

function App() {
  const [weather, setWeather] = useState<WeatherData | null>(null);
  const [loading, setLoading] = useState(true);

  async function getWeather() {
    try {
      const response = await invoke<WeatherData>("fetch_weather");
      setWeather(response);
    } catch (error) {
      console.error("Weather fetch failed!", error);
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    getWeather();
  },[]);

  if (loading) {
    return (<div className="loading-screen">
      <h1>Fetching Weather...</h1>
    </div>)
  }

  if(!weather) {
    return (
      <div className="loading-screen">
        <h1>Failed to load weather data!</h1>
      </div>
    )
  }

  const current = weather.current;

  return (
    <main className="app">
      <section className="weather-card">
        <div className="top-section">
          <div>
            <h1 className="temperature">
              {current.temperature_2m}°C
            </h1>

            <p className="feels-like">
              Feels like {current.apparent_temperature}°C
            </p>
          </div>
          <div className="weather-icon">
            {current.weather_code < 3 ? (
              <Sun size={72} />
            ) : current.weather_code < 60 ? (
              <Cloud size={72} />
            ) : (
              <CloudRain size={72} />
            )}
          </div>
        </div>

        <div className="weather-stats">
          <div className="stat-box">
            <Wind/>
            <span>{current.wind_speed_10m}km/h</span>
          </div>

          <div className="stat-box">
            <Droplets/>
            <span>{current.relative_humidity_2m}%</span>
          </div>

          <div className="stat-box">
            <Thermometer/>
            <span>{current.percipitation}mm</span>
          </div>

          <div className="stat-box">
            <Thermometer/>
            <span>{current.percipitation}mm</span>
          </div>
        </div>

        <section className="forecast-section">
          <h2>7 day Forecast</h2>

          <div className="forecast-grid">
            {weather.daily.time.map((day, index) => (
              <div className="forecast-card" key={day}>
                <p>{day}</p>

                <h3>
                  {weather.daily.temperature_2m_max[index]}° /
                  {weather.daily.temperature_2m_min[index]}°
                </h3>
              </div>
            ))}
          </div>
        </section>
      </section>
    </main>
  );
}

export default App;