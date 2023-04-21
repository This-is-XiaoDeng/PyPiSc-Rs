use std::io::Write;
use std::{path::Path, io};
use std::env;
use colored::Colorize;
use http::{header::USER_AGENT, HeaderMap, HeaderValue};
use reqwest;
pub mod cache;
use scraper::{Html, Selector};

use crate::list::cache::load_cache;

fn get_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.71 Safari/537.36 Edg/94.0.992.38"));
    headers
}

async fn get_package_list() -> Result<Html, reqwest::Error> {
    let html = reqwest::Client::new()
        .get("https://pypi.tuna.tsinghua.edu.cn/simple")
        .headers(get_header())
        .send()
        .await?
        .text()
        .await?;
    Ok(Html::parse_fragment(&html))
}

fn get_python_type(package_name: String, file_name: String) -> String {
    let filename_splited = &mut file_name[package_name.len()..].split("-");
    if filename_splited.clone().count() >= 3 {
        return filename_splited.nth(2).unwrap().to_string();
    } else {
        return "universal".to_string();
    }
}

pub async fn make_cache() -> Result<(), reqwest::Error> {
    let doc = match get_package_list().await {
        Ok(d) => d,
        Err(e) => return Err(e)
    };
    let selector =  Selector::parse("a").unwrap();
    let mut json: Vec<String> = Vec::new();
    for el in doc.select(&selector) {
        json.push(el.inner_html());
    }
    cache::make_cache(json).unwrap();
    Ok(())
}

pub async fn list_package(_selector: String, single_display: u64) -> Result<(), reqwest::Error> {
    print!("Getting packages list ... ");
    io::stdout().flush().unwrap();
    let working_dir: String = env::temp_dir().display().to_string();
    if !Path::new(format!("{}/cache.json", working_dir).as_str()).exists() {
        make_cache().await?;
    }
    let packages = load_cache();
    println!("{}", "Done".green());
    let mut displayed: u64 = 0;

    for package_name in packages {
        // let package_name = el.inner_html();
        if package_name.contains(&_selector) {
            let file_name = get_package_file_name(package_name.clone()).await?;
            let version = get_package_version(package_name.clone(), file_name.clone());
            let python_type = get_python_type(package_name.clone(), file_name.clone());

            println!("{} {} {}", package_name.green(),version, python_type.bright_black());
            displayed += 1;

            if displayed >= single_display {
                print!("Press enter to contiune ...");
                io::stdout().flush().unwrap();
                let mut _input = String::new();
                io::stdin().read_line(&mut _input).unwrap();
                displayed = 0;
            }
        }
    }
    Ok(())
}

async fn get_package_file_name(package_name: String) -> Result<String, reqwest::Error> {
    let html = reqwest::Client::new()
        .get(format!(
            "https://pypi.tuna.tsinghua.edu.cn/simple/{}",
            package_name
        ))
        .headers(get_header())
        .send()
        .await?
        .text()
        .await?;

    let doc = Html::parse_fragment(&html);
    let mut file_name = "".to_string();
    let selector = Selector::parse("a").unwrap();
    let mut _file = "".to_string();
    
    for el in doc.select(&selector) {
        let _file_name = el.inner_html();
        let file_type = _file_name.split(".").last().unwrap();
        if file_type == "whl" {
            file_name = _file_name.to_string();
        }
        _file = _file_name.to_string();
    }

    if file_name == "" {
        file_name = _file.replace(".tar.gz", "").replace(".zip", "");
    }
    Ok(file_name)
}

fn get_package_version(package_name: String, file_name: String) -> String {
    let version = &mut file_name[package_name.len()..]
        .split("-");
    if version.clone().count() >= 2 {
        return version.nth(1)
            .unwrap()
            .to_string();
    } else {
        return "unknown".to_string()
    }   
}
