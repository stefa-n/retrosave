use reqwest::blocking::Client;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::env::args;
use serde_json::json;
use directories::BaseDirs;

fn printhelp() {
    println!("retrosave game [game] [get/save]");
    println!("          e.g. retrosave game pokemon get");
    println!("               retrosave game pokemon save");

    println!("retrosave addgame [game] [filepath] [filename]");
    println!("          e.g. retrosave addgame pokemon /home/user/.local/share/pokemon.sav pokemon.sav");

    println!("retrosave listgames");
    println!("          e.g. retrosave listgames");

    println!("retrosave removegame [game]");
    println!("          e.g. retrosave removegame pokemon");

    println!("retrosave [get|set] [server|name] [value]");
    println!("          e.g. retrosave set server http://localhost:25731");
    println!("               retrosave set name retrosave");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("retrosave v{}", VERSION);

    let binding = BaseDirs::new().unwrap();
    let cache_dir = binding.cache_dir();

    let binding1 = format!("{}/retrosave", cache_dir.to_str().unwrap());
    let cfg = Path::new(&binding1);

    let name;
    let server;

    if !Path::new(&cfg).exists() {
        let mut f = File::create(&format!("{}/retrosave", cache_dir.to_str().unwrap()))?;
        f.write_all(b"{
    \"name\": \"retrosave\",
    \"server\": \"http://localhost:25731\"
}")?;
        name = "retrosave".to_string();
        server = "http://localhost:25731".to_string();
    } else {
        let mut f = File::open(&format!("{}/retrosave", cache_dir.to_str().unwrap()))?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        let data: serde_json::Value = serde_json::from_str(&contents)?;

        name = data["name"].as_str().unwrap().to_string();
        server = data["server"].as_str().unwrap().to_string();
    }

    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        printhelp();
        return Ok(());
    }

    if args[1] == "set" {
        if args[2] == "server" {
            let mut f = File::open(format!("{}/retrosave", cache_dir.to_str().unwrap()))?;
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;

            let mut data: serde_json::Value = serde_json::from_str(&contents)?;
            data["server"] = json!(args[3]);

            let mut f = File::create(&format!("{}/retrosave", cache_dir.to_str().unwrap()))?;
            f.write_all(data.to_string().as_bytes())?;
        } else if args[2] == "name" {
            let mut f = File::open(format!("{}/retrosave", cache_dir.to_str().unwrap()))?;
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;

            let mut data: serde_json::Value = serde_json::from_str(&contents)?;
            data["name"] = json!(args[3]);

            let mut f = File::create(&format!("{}/retrosave", cache_dir.to_str().unwrap()))?;
            f.write_all(data.to_string().as_bytes())?;
        }
        return Ok(());
    }

    if args[1] == "get" {
        if args[2] == "server" {
            println!("{}", server);
        } else if args[2] == "name" {
            println!("{}", name);
        }
        return Ok(());
    }

    if args[1] == "addgame" {
        if args.len() < 5 {
            printhelp();
            return Ok(());
        }

        let mut f = File::open(&format!("{}/retrosave", cache_dir.to_str().unwrap()))?;
        let mut contents = String::new();

        f.read_to_string(&mut contents)?;
        let mut data: serde_json::Value = serde_json::from_str(&contents)?;

        let game = args[2].to_string();
        let filepath = args[3].to_string();
        let filename = args[4].to_string();

        data[game] = json!({
            "filepath": filepath,
            "filename": filename
        });

        let mut f = File::create(&format!("{}/retrosave", cache_dir.to_str().unwrap()))?;
        f.write_all(data.to_string().as_bytes())?;
        return Ok(());
    }

    if args[1] == "removegame" {
        if args.len() < 3 {
            printhelp();
            return Ok(());
        }

        let mut f = File::open(&format!("{}/retrosave", cache_dir.to_str().unwrap()))?;
        let mut contents = String::new();

        f.read_to_string(&mut contents)?;
        let mut data: serde_json::Value = serde_json::from_str(&contents)?;

        let game = args[2].to_string();

        data[game] = json!({});

        let mut f = File::create(&format!("{}/retrosave", cache_dir.to_str().unwrap()))?;
        f.write_all(data.to_string().as_bytes())?;
        return Ok(());
    }

    if args[1] == "listgames" {
        let mut f = File::open(&format!("{}/retrosave", cache_dir.to_str().unwrap()))?;
        let mut contents = String::new();

        f.read_to_string(&mut contents)?;
        let data: serde_json::Value = serde_json::from_str(&contents)?;

        for (key, value) in data.as_object().unwrap() {
            if value.is_object() {
                println!("{}: {}", key, value["filepath"].as_str().unwrap());
            }
        }
        return Ok(());
    }

    if args[1] == "game" {
        if args.len() < 4 {
            printhelp();
            return Ok(());
        }

        let game = args[2].to_string();
        let action = args[3].to_string();

        let client = Client::new();

        let mut f = File::open(&format!("{}/retrosave", cache_dir.to_str().unwrap()))?;
        let mut contents = String::new();

        f.read_to_string(&mut contents)?;
        let data: serde_json::Value = serde_json::from_str(&contents)?;

        if action == "get" {
            let filepath = data[&game]["filepath"].as_str().unwrap();
            let filename = data[&game]["filename"].as_str().unwrap();

            download(&client, &server, &name, &filepath, &filename, &game)?;
        } else if action == "save" {
            let filepath = data[&game]["filepath"].as_str().unwrap();
            let filename = data[&game]["filename"].as_str().unwrap();

            let mut f = File::open(filepath)?;
            let mut save = Vec::new();
            f.read_to_end(&mut save)?;

            upload(&client, &server, &name, &filename, &game, save)?;
        }
    }

    Ok(())
}

fn upload(client: &Client, server: &str, name: &str, filename: &str, game: &str, save: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    client.post(server)
        .body(save)
        .header("username", name)
        .header("filename", filename)
        .header("game", game)
        .send()?;

    Ok(())
}

fn download(client: &Client, server: &str, name: &str, filepath: &str, filename: &str, game: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut res = client.get(server)
        .header("username", name)
        .header("filename", filename)
        .header("game", game)
        .send()?;

    let mut body = Vec::new();
    res.read_to_end(&mut body)?;

    let mut f = File::create(filepath)?;
    f.write_all(&body)?;

    Ok(())
}