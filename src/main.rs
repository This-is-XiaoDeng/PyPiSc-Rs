use colored::Colorize;
mod list;
use std::env;
use reqwest;
mod search;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut args: Vec<_> = env::args().collect();
    // 打印版本信息
    println!(
        "{} {} {}",
        "PyPiSc-Rs".cyan(),
        "[Version: 0.1]".green(),
        "(By This-is-XiaoDeng)".yellow()
    );
    // 解析参数
    if args.len() == 1 {
        println!("Use {} for help.", format!("{} help", args[0]).yellow());
    } else if args[1] == "list" {
        if args.len() == 2 {
            args[2] = "".to_string();
        }
        list::list_package(args[2].clone()).await?;
    } else if args[1] == "help" {
        println!("{} list [name]: List packages by [name]", args[0]);
        println!("{} help: Show this help", args[0]);
        println!("{} update: Update cache", args[0]);
        println!("{} search <name>: Search package on pypi.org", args[0]);
    } else if args[1] == "update" {
        println!("Updating cache ...");
        list::make_cache().await?;
    } else if args[1] == "search" {
        search::search_package(args[2].clone()).await.unwrap();
    }
    Ok(())
}
