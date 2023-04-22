use std::env;
use std::fs;
use std::io;
use std::path;
use std::process;

fn get_music_dir(dir_path:&path::Path) -> io::Result<path::PathBuf>{
    let mut entries:Vec<path::PathBuf> = Vec::new();

    for (index, entry) in fs::read_dir(dir_path)?.enumerate() {
        let entry = entry?;
        let path = entry.path();
        
        println!("{index}. {}", path.display());
        entries.push(path);
    }

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim();

    if choice == "" {
        return Ok(dir_path.to_path_buf());
    }

    let choice:usize = choice.parse().unwrap();
    return get_music_dir(entries[choice].as_path());
}

fn visit_dirs(dir:&path::Path, files:&mut Vec<String>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(path.as_path(), files)?;
            } else {
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

    println!("Filepath {file_path}");
    let music_dir = get_music_dir(path::Path::new(file_path)).unwrap();
    println!("{}", music_dir.display());

    let mut files:Vec<String> = Vec::new();
    visit_dirs(music_dir.as_path(), &mut files);

    for f in &files {
        println!("{}", f);
    }
    let mut child = process::Command::new(player)
        .args(files)
        .spawn()
        .expect("failed to execute process");
    child.wait().expect("failed to wait on child");

}
