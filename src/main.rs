use reqwest::blocking::{get, Response};
use serde::Deserialize;
use std::{
    env::args,
    fs::{create_dir_all, File},
    path::Path,
};

const GAMES_URL: &str = "https://stb-gaming.github.io/high-scores/games.json";

#[derive(Deserialize)]
struct Game {
    // title:String,
    // description:String,
    // developer:String,
    // service:String,
    // url:String,
    // portal:String,
    highscores: String,
    // submit:String,
    // brand:String,
    // splash:String,
    // gameplay:String,
    // date:String,
    // archived:String,
    // image:String,
    // menu:String,
    // list:String,
    // category:String,
    files: Vec<String>,
}

fn download(url: String) -> Response {
    let response = get(url).expect(&("Error fetching URL"));
    response
}

fn get_game_list() -> Vec<Game> {
    println!("Getting games information from {}", GAMES_URL);
    let games_json: Vec<Game> = download(GAMES_URL.to_string())
        .json()
        .expect("Failed to parse JSON");
    games_json
}

fn download_game(game: &Game, out: &str) {
    let path = Path::new(out);
    if !std::fs::exists(path).expect("Unable to find out if file exists") {
        let _ = create_dir_all(path);
        for file_url in &game.files {
            let file_name = Path::new(file_url).file_name().unwrap().to_str().unwrap();
            let file_path = path.join(file_name);
            println!("{} => {}", file_url, file_path.display());
            let mut file = File::create(file_path).unwrap();
            download(file_url.to_string())
                .copy_to(&mut file)
                .expect("Failed to download file");

            //let game_file_out = Path::new(game_file_url).file_name().unwrap().to_str().unwrap();
            //let mut game_file = File::create(game_file_out).expect("Failed to create file");
            //download(game_file_url.to_string()).copy_to(&mut game_file).expect("Failed to download file");
        }
    }
}

fn get_game(id: &str) -> Result<Game, ()> {
    let games = get_game_list();
    let hs_url = format!("https://stb-gaming.github.io/high-scores/games/{}/", id);

    for game in games {
        if game.highscores == hs_url {
            return Ok(game);
        }
    }
    return Err(());
}

fn main() {
    let args: Vec<String> = args().collect();

    let mut action = "help";

    if args.len() > 1 {
        action = args[1].as_str();
    }

    match action {
        "a" => {
            println!("yay")
        }
        "download" | "dl" => {
            if args.len() < 3 {
                print!("{} download (gameid) [out]", args[0]);
                return;
            }
            let id = args[2].as_str();
            let mut out = id;
            if args.len() > 3 {
                out = args[3].as_str();
            }

            println!("Obtaining game with id {}", id);
            let game = get_game(id).expect("Could not obtain game");
            download_game(&game, out);
        }
        _ => {
            println!("todo: help page");
        }
    }
}
