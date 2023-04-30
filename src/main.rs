use colored::Colorize;
mod list;
use std::{env, u64::MAX, process::Command};
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
    let mut index_url: String = "https://pypi.tuna.tsinghua.edu.cn/simple".to_string();
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
            }
            "--index" | "-i" => {
                if i != args.len() - 1 {
                    index_url = args[i+1].clone();
                } else {
                    println!("Usage: {} {}", "--index".green(), "<url>".red())
                }
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
                } else if args[i].contains("-i") {
                    index_url = args[i].replace("-i", "");
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
        list::list_package(args[2].clone(), single_display, index_url).await?;
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
        println!("\t--index <url> (-i)              - Set index url");
    } else if args[1] == "update" {
        println!("Updating cache ...");
        list::make_cache(&index_url).await?;
    } else if args[1] == "search" {
        search::search_package(args[2].clone(), single_display).await.unwrap();
    } else {
        let mut cmd = Command::new("python3");
        cmd.arg("-m").arg("pip").args(&args[1..]);
        let output = cmd.output().unwrap();
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    Ok(())
}
