
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn parse_anime_dir(anime_dir: &str) -> io::Result<()> {
    
    let mut path = PathBuf::from(anime_dir);
    for f in fs::read_dir(path)? {
        let f = f?;
        let meta = f.metadata()?;
        println!("{:?}", f.file_name());
    }
    Ok(())
}
