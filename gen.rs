
// generate static pages

use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::{BufWriter, Write};

use super::config;
use super::common::{BoxResult};
use super::config::{UserConfig};
use super::parse::{AnimeData};

pub fn generate_all(data: &Vec<AnimeData>, user_config: &UserConfig) -> BoxResult<()> {

    generate_index_page(data, user_config)?;

    Ok(())
}

fn generate_index_page(data: &Vec<AnimeData>, user_config: &UserConfig) -> BoxResult<()> {

    // TODO this function is very ugly looking lmao

    let index_path = &user_config.static_path.join("index.html");
    let f = File::create(index_path)?;
    let mut writer = BufWriter::new(f);

    let html = r###"<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <link rel="stylesheet" href="../styles/index.css">
        <link rel="shortcut icon" href="#">
        <title>ramen | からつばLABS</title>
    </head>

    <body>
"###;
    writer.write_all(html.as_bytes())?;

    write_navbar(&mut writer, data, user_config)?;

    let html = r###"<div class="index-main-container">
            <div class="title-container">
                <h1>🍜 the ramen index</h1>
                <hr>
                <p>the <b>ramen</b> project is a collection of anime, manga and other content hosted by <b>からつばLABS</b> and ready for download or streaming
                </p>
                <hr>
            </div>
            <ul>
"###;
    writer.write_all(html.as_bytes())?;

    for anime_data in data {
        write_index_entry(&mut writer, anime_data, user_config)?;
    }

    let html = r###"</ul>
        </div>
    </body>
</html>
"###;
    writer.write_all(html.as_bytes())?;
    
    Ok(())
}

fn write_navbar(writer: &mut BufWriter<File>, data: &Vec<AnimeData>, user_config: &UserConfig) -> BoxResult<()> {

    // TODO maybe read these html snippets from file instead
    let html = format!(r###"<div class="navbar">
        <a class="app-logo-text" href="{site_url}">ラーメン</a> <a class="karat-text" href="https://github.com/KaratsubaLabs">BY からつばLABS</a>
    </div>
"###, site_url = &user_config.site_url);
    writer.write_all(html.as_bytes())?;

    Ok(())
}

fn write_index_entry(writer: &mut BufWriter<File>, anime_data: &AnimeData, user_config: &UserConfig) -> BoxResult<()> {

    let html = format!(r###"<a href="#"><li>{title}</li></a>
"###, title = &anime_data.meta.title);
    writer.write_all(html.as_bytes())?;

    Ok(())
}

