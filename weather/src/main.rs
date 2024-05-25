use std::io;
use serde::Deserialize;
use colored::*;

#[derive(Deserialize, Debug)]
struct WeatherResponse
{
    weather: Vec<Weather>,
    main : Main,
    wind: Wind,
    name: String
}

#[derive(Deserialize, Debug)]
struct Weather
{
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main
{
    temp: f64,
    humidity: f64,
    pressure: f64
}

// Add the wind information entity //

fn main() {
    println!("Hello, world!");
    println!("Weather App.")
}
