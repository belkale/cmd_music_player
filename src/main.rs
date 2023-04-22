use std::env;
use std::fs;
use std::io;
use std::path;
use std::process;

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
fn main() {
    let args: Vec<String> = env::args().collect();
    let player = &args[1];
    let file_path = &args[2];

    let music_dir = get_music_dir(path::Path::new(file_path))
        .expect("failed to browse music directory");
    if let None = music_dir {
        return;
    }
    let music_dir = music_dir.unwrap();
    println!("Playing from {}", music_dir.display());

    let mut files:Vec<String> = Vec::new();
    visit_dirs(music_dir.as_path(), &mut files)
        .expect("failed to list directory");

    for f in &files {
        println!("{}", f);
    }
    let mut child = process::Command::new(player)
        .args(files)
        .spawn()
        .expect("failed to execute process");
    child.wait().expect("failed to wait on child");

}
