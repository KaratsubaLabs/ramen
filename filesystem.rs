
use std::fs;
use std::fs::File;
use std::path::{PathBuf, Path};
use std::io;
use std::io::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum AnimeType {
    TV,
    OVA,
    Movie
}

#[derive(Debug)]
pub struct AnimeData {
    pub meta: AnimeMeta,
    pub episodes: BTreeMap<u8, EpisodeData>
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
    // cover image
    // release data
    // tags
}

// TODO: make this better and less redundant
#[derive(Debug)]
struct AnimeMetaBuiilder {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub synopsis: Option<String>,
    pub anime_type: Option<AnimeType>,
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
            anime_type: None
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

    pub fn build(self) -> AnimeMeta {
        // TODO handle the unwrap panics
        AnimeMeta {
            title: self.title.unwrap(),
            original_title: self.original_title,
            synopsis: self.synopsis.unwrap(),
            anime_type: self.anime_type.unwrap(),
        }
    }
}

pub fn parse_all(root_dir: &str) -> io::Result<()> {
    
    let path = PathBuf::from(root_dir);
    for f in fs::read_dir(path)? {
        let f = f?;
        println!("parsing {:?}", f.file_name());
        if f.path().is_dir() {
            parse_anime(&f.path());
        }
    }
    Ok(())
}

fn parse_anime(anime_dir: &Path) -> io::Result<AnimeData> {

    // grab metadata
    let path = anime_dir.join("data");
    let meta = parse_meta(&path.join("metadata"))?;

    // grab list of all files
    let path = anime_dir.join("files");
    let mut episodes: BTreeMap<u8, EpisodeData> = BTreeMap::new();
    for f in fs::read_dir(path)? {
        let f = f?;

        if !f.path().is_file() {
            continue;
        }

        // grab file name and extension
        let raw_filename = f.file_name().into_string();
        if raw_filename.is_err() {
            continue;
        }
        let raw_filename = raw_filename.unwrap();
        let split = raw_filename.rsplit_once(".");
        if split.is_none() {
            continue;
        }
        let split = split.unwrap();

        // TODO check if file is video file

        // TODO get duration of video

        let episode_number = split.0.parse::<u8>();
        if episode_number.is_err() {
            continue;
        }
        let episode_number = episode_number.unwrap();

        let episode_data = EpisodeData::new();
        episodes.insert(episode_number, episode_data);
    }

    // grab list of all subtitles
    let path = anime_dir.join("subtitles");
    for f in fs::read_dir(path)? {
    }

    let anime_data = AnimeData {
        meta: meta,
        episodes: episodes
    };

    Ok(anime_data)
}

fn parse_meta(meta_path: &Path) -> io::Result<AnimeMeta> {

    let file = File::open(meta_path)?;
    let reader = io::BufReader::new(file);

    let mut builder = AnimeMetaBuiilder::new();

    // TODO: reading lines from file could be more optimized
    for line in reader.lines() {
        let line = line?;
        let split = line.split_once("=");
        if split.is_none() {
            // TODO: possibly warn invalid config
            continue;
        }
        let mut split = split.unwrap();
        split.0 = split.0.trim();
        split.1 = split.1.trim();

        println!("{:?}", split);

        builder = match split.0 {
            "title" => builder.title(split.1.to_string()),
            "original_title" => builder.original_title(split.1.to_string()),
            "synopsis" => builder.synopsis(split.1.to_string()),
            "anime_type" => builder.anime_type(split.1.to_string()),
            _ => builder
        }
    }

    let anime_data = builder.build();
    println!("{:?}", anime_data);

    Ok(anime_data)
}

