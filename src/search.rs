use std::io::{self, Write};

use reqwest;
use scraper::{Html, Selector};
use colored::Colorize;

async fn get_page(name: &String, page: u16) -> Result<Html, reqwest::Error> {
    let html = reqwest::Client::new()
        .get(format!("https://pypi.org/search/?q={}&page={}", name, page))
        .send()
        .await?
        .text()
        .await?;
    Ok(Html::parse_fragment(&html))  
}

fn get_packages(page: Html) -> Result<Vec<Html>, String> {
    let selector = Selector::parse(".package-snippet").unwrap();
    let mut package_list: Vec<Html> = Vec::new();
    for el in page.select(&selector) {
        let html = el.inner_html();
        package_list.push(Html::parse_fragment(&html));
    }
    Ok(package_list)
}

fn get_package_data(package: &Html, data: &'static str) -> Result<String, ()> {
    let selector = Selector::parse(format!(".package-snippet__{}", data).as_str()).unwrap();
    for el in package.select(&selector) {
        return Ok(el.inner_html().to_string());
    }
    return Err(())
}

fn get_create_time(package: &Html) -> Result<String, ()> {
    let data = get_package_data(&package, "created").unwrap();
    let doc = Html::parse_fragment(&data);
    let selector = Selector::parse("time").unwrap();
    for el in doc.select(&selector) {
        return Ok(el.inner_html().replace("\n", "").replace("  ", ""));
    }
    return Err(());
}

pub async fn search_package(name: String, single_display: u64) -> Result<(), ()> {
    let mut p = 1;
    let mut displayed = 0;
    loop {
        // println!("getting page {}", p);
        let page = get_page(&name, p).await.unwrap();
        let packages = get_packages(page).unwrap();
        if packages.len() == 0 {
            break
        }
        for package in packages {
            let name = get_package_data(&package, "name").unwrap();
            let version = get_package_data(&package, "version").unwrap();
            let description = get_package_data(&package, "description").unwrap();
            let created = get_create_time(&package).unwrap();
            println!("{} {} {}", name.green(), version, created.bright_black());
            println!("  {}\n", description);

            displayed += 1;
            if displayed >= single_display {
                print!("Press enter to contiune ...");
                io::stdout().flush().unwrap();
                let mut _input = String::new();
                io::stdin().read_line(&mut _input).unwrap();
                displayed = 0;
            }
        }
        p += 1;
    }
    Ok(())
}
