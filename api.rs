
// api requests

use crate::serde::{Deserialize};

pub static API_URL: &str = "https://api.jikan.moe/v4";

#[derive(Deserialize, Debug)]
struct GetAnimeResponse {
    pub data: AnimeData
}

#[derive(Deserialize, Debug)]
struct SearchAnimeResponse {
    pub data: Vec<AnimeData>
}

#[derive(Deserialize, Debug)]
struct AnimeData {
    pub mal_id: u32,
    pub title_english: String,
    pub title_japanese: String,
    pub synopsis: String,
}

pub fn get_anime_meta(anime_id: &str) -> Result<(), reqwest::Error> {

    let resp = reqwest::blocking::get(format!("{}/anime/{}", API_URL, anime_id))?.json::<GetAnimeResponse>()?;

    println!("{:?}", resp);

    Ok(())
}

pub fn search_anime(anime_name: &str) -> Result<(), reqwest::Error> {

    let resp = reqwest::blocking::get(format!("{}/anime?q={}&limit=5", API_URL, anime_name))?.json::<SearchAnimeResponse>()?;

    println!("{:?}", resp);

    Ok(())

}

