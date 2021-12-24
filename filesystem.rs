
use std::fs;
use std::fs::File;
use std::path::{PathBuf, Path};
use std::io;
use std::io::prelude::*;

enum AnimeType {
    TV,
    Movie
}

#[derive(Debug)]
pub struct AnimeData {
    pub meta: AnimeMeta
}

#[derive(Debug)]
pub struct AnimeMeta {
    pub title: String,
    pub original_title: Option<String>,
    pub synopsis: String,
}

// TODO: make this better and less redundant
#[derive(Debug)]
struct AnimeMetaBuiilder {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub synopsis: Option<String>,
}

impl AnimeMetaBuiilder {
    pub fn new() -> AnimeMetaBuiilder {
        AnimeMetaBuiilder {
            title: None,
            original_title: None,
            synopsis: None
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

    pub fn build(self) -> AnimeMeta {
        // TODO handle the unwrap panics
        AnimeMeta {
            title: self.title.unwrap(),
            original_title: self.original_title,
            synopsis: self.synopsis.unwrap()
        }
    }
}

pub fn parse_all(root_dir: &str) -> io::Result<()> {
    
    let path = PathBuf::from(root_dir);
    for f in fs::read_dir(path)? {
        let f = f?;
        let meta = f.metadata()?;
        println!("parsing {:?}", f.file_name());
        if f.path().is_dir() {
            parse_anime(&f.path());
        }
    }
    Ok(())
}

fn parse_anime(anime_dir: &Path) -> io::Result<()> {

    // grab metadata
    let path = anime_dir.join("data");
    parse_meta(&path.join("metadata"));

    // grab list of all files
    let path = anime_dir.join("files");
    for f in fs::read_dir(path)? {

    }

    // grab list of all subtitles
    let path = anime_dir.join("subtitles");
    for f in fs::read_dir(path)? {

    }

    Ok(())
}

fn parse_meta(meta_path: &Path) -> io::Result<()> {

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

        match split.0 {
            "title" => builder = builder.title(split.1.to_string()),
            "original_title" => builder = builder.original_title(split.1.to_string()),
            "synopsis" => builder = builder.synopsis(split.1.to_string()),
            _ => ()
        }
    }

    let anime_data = builder.build();
    println!("{:?}", anime_data);

    Ok(())
}

