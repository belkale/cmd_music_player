
use std::env;
use std::process;

use cmd_music_player::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:{err}.
Usage: cmd_music_player <player> <music_dir_path>");
        process::exit(1);
    });

    if let Err(e) = cmd_music_player::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}
