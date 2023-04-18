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
        println!("Usage: {} {{help|list|update|search}} [arguments...]", args[0]);        
        println!("\nCommands:");
        println!("\tlist [name]      - List modules");
        println!("\tsearch <keyword> - Search modules on pypi.org");
        println!("\tupdate           - Update modules cache");
        println!("\thelp             - Show this help");
        println!("\nArguments:");
        println!("\tname    - Package name");
        println!("\tkeyword - Key word");
        println!("\nOptions:");
        println!("\t--single-display <count> (-s)   - Single display packages count");
        

    } else if args[1] == "update" {
        println!("Updating cache ...");
        list::make_cache().await?;
    } else if args[1] == "search" {
        search::search_package(args[2].clone(), single_display).await.unwrap();
    }
    Ok(())
}
