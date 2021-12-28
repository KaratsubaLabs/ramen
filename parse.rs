
// parses file system

use std::fs;
use std::fs::File;
use std::ffi::OsString;
use std::path::{PathBuf, Path};
use std::io;
use std::io::prelude::*;
use std::collections::BTreeMap;

use super::common::{BoxResult};
use super::error::{SimpleError};

#[derive(Debug)]
pub enum AnimeType {
    TV,
    OVA,
    Movie
}

#[derive(Debug)]
pub struct AnimeData {
    pub meta: AnimeMeta,
    pub episodes: BTreeMap<u8, EpisodeData>,
    pub dir_name: OsString,
}

#[derive(Debug)]
pub struct EpisodeData {
    pub name: Option<String>,
    pub subtitles: Vec<SubtitleData>,
    // duration
}

#[derive(Debug)]
pub struct SubtitleData {
    pub language: String,
    // subtitle file
}

#[derive(Debug)]
pub struct AnimeMeta {
    pub title: String,
    pub original_title: Option<String>,
    pub synopsis: String,
    pub anime_type: AnimeType,
    pub img_url: Option<String>, // this can also be a url type maybe
    pub release_year: Option<String>,
    pub tags: Option<Vec<String>>,
}

// TODO: make this better and less redundant
#[derive(Debug)]
struct AnimeMetaBuiilder {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub synopsis: Option<String>,
    pub anime_type: Option<AnimeType>,
    pub img_url: Option<String>,
    pub release_year: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl EpisodeData {
    pub fn new() -> Self {
        EpisodeData{
            name: None,
            subtitles: Vec::new()
        }
    }
}

impl AnimeMetaBuiilder {
    pub fn new() -> AnimeMetaBuiilder {
        AnimeMetaBuiilder {
            title: None,
            original_title: None,
            synopsis: None,
            anime_type: None,
            img_url: None,
            release_year: None,
            tags: None,
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
    pub fn original_title(mut self, original_title: String) -> Self {
        self.original_title = Some(original_title);
        self
    }
    pub fn synopsis(mut self, synopsis: String) -> Self {
        self.synopsis = Some(synopsis);
        self
    }
    pub fn anime_type(mut self, anime_type: String) -> Self {
        let anime_type = anime_type.to_lowercase();
        self.anime_type = match &anime_type as &str {
            "tv" => Some(AnimeType::TV),
            "ova" => Some(AnimeType::OVA),
            "movie" => Some(AnimeType::Movie),
            _ => None
        };
        self
    }
    pub fn img_url(mut self, img_url: String) -> Self {
        self.img_url = Some(img_url);
        self
    }
    pub fn release_year(mut self, release_year: String) -> Self {
        self.release_year = Some(release_year);
        self
    }
    pub fn tags(mut self, tags: String) -> Self {
        let tags_list: Vec<String> = tags.split(",").map(|tag| tag.trim().to_owned()).collect();
        self.tags = Some(tags_list);
        self
    }

    pub fn build(self) -> Option<AnimeMeta> {
        Some(AnimeMeta {
            title: self.title.unwrap(),
            original_title: self.original_title,
            synopsis: self.synopsis.unwrap(),
            anime_type: self.anime_type.unwrap(),
            img_url: self.img_url,
            release_year: self.release_year,
            tags: self.tags,
        })
    }
}

pub fn parse_all(path: &Path) -> BoxResult<Vec<AnimeData>> {
    
    let mut anime_data_index = Vec::new();
    for f in fs::read_dir(path)? {
        let f = f?;
        println!("parsing {:?}", f.file_name());
        if !f.path().is_dir() { continue; }

        let anime_data = parse_anime(&f.path());
        if anime_data.is_err() { continue; }

        anime_data_index.push(anime_data.unwrap());
    }
    Ok(anime_data_index)
}

fn parse_anime(anime_dir: &Path) -> BoxResult<AnimeData> {

    // grab metadata
    let path = anime_dir.join("data");
    let meta = parse_meta(&path.join("metadata"))?;

    // grab list of all files
    let path = anime_dir.join("files");
    let mut episodes: BTreeMap<u8, EpisodeData> = BTreeMap::new();
    for f in fs::read_dir(path)? {
        if f.is_err() { continue; }
        let f = f?;

        let episode = parse_episode_file(&f.path());
        if episode.is_none() { continue; }

        let episode = episode.unwrap();
        episodes.insert(episode.0, episode.1);
    }

    // grab list of all subtitles
    let path = anime_dir.join("subtitles");
    for f in fs::read_dir(path)? {
        let f = f?;

        let subtitle = parse_subtitle_file(&f.path());
        if subtitle.is_none() { continue; }
        let subtitle = subtitle.unwrap();

        // check if subtitle file has a corresponding episode
        if !episodes.contains_key(&subtitle.0) { continue; }

        let ep_data = episodes.get_mut(&subtitle.0).unwrap();
        ep_data.subtitles.push(subtitle.1);
    }

    let dir_name = anime_dir.file_name().ok_or(SimpleError::new("failed to parse dir_name"))?;

    let anime_data = AnimeData {
        meta: meta,
        episodes: episodes,
        dir_name: dir_name.to_os_string()
    };

    println!("{:?}", anime_data);

    Ok(anime_data)
}

fn parse_meta(meta_path: &Path) -> BoxResult<AnimeMeta> {

    let file = File::open(meta_path)?;
    let reader = io::BufReader::new(file);

    let mut builder = AnimeMetaBuiilder::new();

    // TODO: reading lines from file could be more optimized
    for line in reader.lines() {
        let line = line?;
        let split = line.split_once("=");
        if split.is_none() { continue; }

        let mut split = split.unwrap();
        split = (split.0.trim(), split.1.trim());

        builder = match split.0 {
            "title" => builder.title(split.1.to_string()),
            "original_title" => builder.original_title(split.1.to_string()),
            "synopsis" => builder.synopsis(split.1.to_string()),
            "anime_type" => builder.anime_type(split.1.to_string()),
            "img_url" => builder.img_url(split.1.to_string()),
            _ => builder
        }
    }

    let anime_data = builder.build().ok_or(SimpleError::new("failed to build anime data"))?;
    println!("{:?}", anime_data);

    Ok(anime_data)
}

fn parse_episode_file(ep_file: &Path) -> Option<(u8,EpisodeData)> {

    if !ep_file.is_file() { return None; }

    // grab file name and extension
    let episode_number = ep_file.file_stem()?.to_str()?.parse::<u8>().ok()?;
    let extension = ep_file.extension()?.to_str()?;

    let mut episode_data = EpisodeData::new();

    // TODO check if file is video file

    // TODO get duration of video

    Some((episode_number, episode_data))
}

// subtitle file name in form [episode_number]_[language].[extension]
fn parse_subtitle_file(sub_file: &Path) -> Option<(u8,SubtitleData)> {

    if !sub_file.is_file() { return None; }

    // grab file name and extension
    let filename = sub_file.file_name()?.to_str()?;
    let split = filename.rsplit_once(".")?;
    let split = split.0.split_once("_")?;

    let episode_number = split.0.parse::<u8>().ok()?;
    let language = split.1;

    let subtitle_data = SubtitleData{
        language: language.to_string()
    };
    Some((episode_number,subtitle_data))
}

