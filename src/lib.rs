use std::fs;
use std::io;
use std::path;
use std::process;
use std::error::Error;

pub struct Config {
    pub player: String,
    pub music_dir: String,
}

impl Config {
    pub fn build(args:&[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let player = args[1].clone();
        let music_dir = args[2].clone();
        Ok(Config{player, music_dir})
    }
}

fn get_music_dir(dir_path:&path::Path) -> io::Result<Option<path::PathBuf>>{
    let mut entries:Vec<path::PathBuf> = Vec::new();

    println!("Listing directory {}", dir_path.display());
    for (index, entry) in fs::read_dir(dir_path)?.enumerate() {
        let entry = entry?;
        let path = entry.path();
        
        println!("{index}. {}", path.display());
        entries.push(path);
    }

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim();
    
    match choice {
        "" => Ok(Some(dir_path.to_path_buf())),
        "q" => Ok(None),
        "u" => get_music_dir(dir_path.parent().unwrap()),
        num => {
            let num:usize = num.parse().unwrap();
            get_music_dir(entries[num].as_path())
        }
    }
}

fn visit_dirs(dir:&path::Path, files:&mut Vec<String>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push(path.into_os_string().into_string().unwrap());
            }
        }
    }
    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let music_dir = get_music_dir(path::Path::new(&config.music_dir))?;
    if let None = music_dir {
        return Ok(());
    }
    let music_dir = music_dir.unwrap();
    println!("Playing from {}", music_dir.display());

    let mut files:Vec<String> = Vec::new();
    visit_dirs(music_dir.as_path(), &mut files)?;

    for f in &files {
        println!("{}", f);
    }
    let mut child = process::Command::new(&config.player)
        .args(files)
        .spawn()?;
    child.wait()?;
    Ok(())
}