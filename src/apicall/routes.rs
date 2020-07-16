use actix_web::{ get, web, HttpResponse};
use reqwest::Url;
use std::env;
use serde_derive::{Deserialize, Serialize};
use crate::error_handler::ResponseErrorWrapper;


#[get("todayweatherbycity/{city}/{country_code}")]
async fn get_today_weather_by_city_id(place: web::Path<(String,String)>) -> Result<HttpResponse, ResponseErrorWrapper>
{
    let weather = Forecast::today_weather_by_city(place.into_inner()).await?;

    Ok(HttpResponse::Ok().json(weather))
}

pub fn init_routes(config: &mut web::ServiceConfig){
    config.service(get_today_weather_by_city_id);
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}

impl Forecast {
    async fn today_weather_by_city(place: (String,String)) -> Result<Self, anyhow::Error> {
        let url = format!("http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}", place.0, place.1, env::var("APP_ID").unwrap());
        let url = Url::parse(&*url)?;
        let weathers = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;
        Ok(weathers)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details
}
#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}
