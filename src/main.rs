use colored::Colorize;
mod list;
use std::{env, u64::MAX};
use reqwest;
mod search;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut args: Vec<_> = env::args().collect();
    
    println!(
        "{} {} {}",
        "PyPiSc-Rs".cyan(),
        "[Version: 1.0]".green(),
        "(By This-is-XiaoDeng)".yellow()
    );
    
    let mut single_display: u64 = MAX;
    for i in 0..args.len() {
        match args[i].as_str() {
            "--single-display" | "-s" => {
                if i != args.len() - 1 {
                    single_display = match args[i].parse::<u64>() {
                        Ok(int) => int,
                        Err(e) => {
                            println!("Unknown argument {}: {}", args[i].yellow(), e.to_string());
                            break
                        }
                    };
                } else {
                    println!("Usage: {} {}", "--single-display".green(), "<count>".red());
                }
                break
            }
            _ => {
                if args[i].contains("-s") {
                    single_display = match args[i].replace("-s", "").as_str().parse::<u64>() {
                        Ok(int) => int,
                        Err(e) => {
                            println!("Unknown argument {}: {}", args[i].yellow(), e.to_string());
                            break
                        }
                    }
                }
            }
        }
    }

    if args.len() == 1 {
        println!("Use {} for help.", format!("{} help", args[0]).yellow());
    } else if args[1] == "list" {
        if args.len() == 2 {
            args[2] = "".to_string();
        }
        list::list_package(args[2].clone(), single_display).await?;
    } else if args[1] == "help" {
        println!("{} list [name]: List packages by [name]", args[0]);
        println!("{} help: Show this help", args[0]);
        println!("{} update: Update cache", args[0]);
        println!("{} search <name>: Search package on pypi.org", args[0]);
    } else if args[1] == "update" {
        println!("Updating cache ...");
        list::make_cache().await?;
    } else if args[1] == "search" {
        search::search_package(args[2].clone(), single_display).await.unwrap();
    }
    Ok(())
}
