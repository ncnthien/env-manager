use clap::Parser;
use std::path::Path;
use std::fs;

#[derive(Parser)]
struct Cli {
    command: String,
    parameter: String
}

fn switch(env: &str) {
    let is_env_name_valid = ["dev", "uat", "sit"].contains(&env);

    if !is_env_name_valid {
        panic!("Only support dev, uat, sit!");
    }

    let file_name = format!(".env.{}", env);
    let file_path = Path::new(&file_name);

    if !file_path.exists() {
        panic!("{:?} is not found!", file_path);
    }

    let file_content = fs::read_to_string(file_path);

    match file_content {
        Ok(content) => {
            let env_path = Path::new(".env");
            match env_path.exists() {
                true => { fs::File::open(env_path).unwrap() },
                false => { fs::File::create(env_path).unwrap() }
            };

            match fs::write(env_path, content) {
                Ok(_) => println!("Switch to env!"),
                Err(error) => println!("Cannot write to .env, the error: {}", error)
            }
        },
        Err(error) => {
            panic!("Cannot read file {:?}, the error is: {}", file_path, error);
        }
    }
    
    
}

fn token(content: &str) {
    println!("The token is: {:?}", content);
}

fn main() {
    let args = Cli::parse();

    match args.command.as_str() {
        "switch" => switch(&args.parameter),
        "token" => token(&args.parameter),
        _ => panic!("command not found!")
    }
}
