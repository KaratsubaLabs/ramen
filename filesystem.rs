
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

pub fn parse_anime(anime_dir: &Path) -> io::Result<()> {

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

pub fn parse_meta(meta_path: &Path) -> io::Result<()> {

    let file = File::open(meta_path)?;
    let reader = io::BufReader::new(file);

    // TODO: reading lines from file could be more optimized
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

