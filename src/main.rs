use chrono::{Local, TimeZone};
use clap::Parser;
use colored::*;
use reqwest;
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "weather")]
#[command(about = "Weather CLI", long_about = None)]
struct Args {
    #[arg()]
    location: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WeatherData {
    pub main: Main,
    pub weather: Vec<Weather>,
    pub sys: Sys,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Sys {
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Weather {
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ForecastData {
    pub list: Vec<List>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct List {
    pub weather: Vec<Weather>,
    pub main: Main,
    pub dt: i64,
}

async fn get_weather(location: String) -> reqwest::Result<WeatherData> {
    reqwest::get(format!("https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&cnt=8&appid=7484f462f852c04cbab6a6a5ad8c9d37", location))
        .await?
        .json::<WeatherData>()
        .await
}

async fn get_forecast(location: String) -> reqwest::Result<ForecastData> {
    reqwest::get(format!("https://api.openweathermap.org/data/2.5/forecast?q={}&units=metric&cnt=8&appid=7484f462f852c04cbab6a6a5ad8c9d37", location))
        .await?
        .json::<ForecastData>()
        .await
}

async fn print_weather(location: &str) {
    match get_weather(location.to_string()).await {
        Ok(weather_data) => {
            println!(
                "{}: {}°C (feels like {}°C) - {}",
                "status".bold().blue(),
                weather_data.main.temp.round(),
                weather_data.main.feels_like.round(),
                weather_data.weather[0].description
            );

            let sunrise = Local.timestamp(weather_data.sys.sunrise, 0);
            let sunset = Local.timestamp(weather_data.sys.sunset, 0);
            println!("{}: {}", "sunrise".bold().blue(), sunrise.format("%H:%M"));
            println!("{}: {}", "sunset".bold().blue(), sunset.format("%H:%M"));

            // Fetch and display forecast data
            match get_forecast(location.to_string()).await {
                Ok(forecast_data) => {
                    println!("\n{}", "forecast:".bold().cyan());

                    for entry in forecast_data.list.iter().take(8) {
                        let dt = Local.timestamp(entry.dt, 0);
                        println!(
                            "{} - {}°C (feels like {}°C) - {}",
                            dt.format("%H:%M"),
                            entry.main.temp.round(),
                            entry.main.feels_like.round(),
                            entry.weather[0].description
                        );
                    }
                }
                Err(e) => eprintln!("{}: {}", "Error fetching forecast".bold().red(), e),
            }
        }
        Err(e) => eprintln!("{}: {}", "Error fetching weather".bold().red(), e),
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    print_weather(&args.location).await;
}
