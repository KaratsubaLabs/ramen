
// api requests

pub static API_URL: &str = "https://api.jikan.moe/v4";

pub fn search_anime(anime_name: &str) -> Result<(), reqwest::Error> {

    let resp = reqwest::blocking::get(format!("{}/anime?q={}&limit=1", API_URL, anime_name))?.text()?;
    println!("{:?}", resp);

    Ok(())
}

