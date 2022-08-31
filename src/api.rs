// api requests

use super::{
    common::BoxResult,
    error::SimpleError,
    parse::{AnimeMeta, AnimeMetaBuiilder},
};
use crate::serde::Deserialize;

pub static API_URL: &str = "https://api.jikan.moe/v4";

#[derive(Deserialize, Debug)]
struct GetAnimeResponse {
    pub data: APIAnimeData,
}

#[derive(Deserialize, Debug)]
struct SearchAnimeResponse {
    pub data: Vec<APIAnimeData>,
}

#[derive(Deserialize, Debug)]
struct APIAnimeData {
    pub mal_id: u32,
    pub title_english: String,
    pub title_japanese: String,
    pub synopsis: String,
    #[serde(rename = "type")]
    pub anime_type: String,
}

pub fn get_anime_by_id(anime_id: &str) -> BoxResult<AnimeMeta> {
    let resp = reqwest::blocking::get(format!("{}/anime/{}", API_URL, anime_id))?
        .json::<GetAnimeResponse>()
        .or(Err(SimpleError::new("failed making api call")))?;

    // println!("{:?}", resp.data);

    let anime_meta = to_anime_meta(resp.data)?;

    Ok(anime_meta)
}

/*
pub fn search_anime(anime_name: &str) -> Result<(), reqwest::Error> {

    let resp = reqwest::blocking::get(format!("{}/anime?q={}&limit=5", API_URL, anime_name))?.json::<SearchAnimeResponse>()?;

    println!("{:?}", resp);

    Ok(())
}
*/

fn to_anime_meta(anime_data: APIAnimeData) -> BoxResult<AnimeMeta> {
    let builder = AnimeMetaBuiilder::new();

    let anime_meta = builder
        .title(anime_data.title_english)
        .original_title(anime_data.title_japanese)
        .synopsis(anime_data.synopsis)
        .anime_type(anime_data.anime_type)
        .build()
        .ok_or(SimpleError::new("failed to parse anime meta from api"))?;

    Ok(anime_meta)
}
