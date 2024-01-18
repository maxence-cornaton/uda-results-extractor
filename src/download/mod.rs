use std::fs::{create_dir_all, File};
use std::io::{Error, ErrorKind, Write};

use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};
use tokio::join;

use crate::utils::credentials::Credentials;

const CONVENTIONS_TAG: [&str; 2] = [
    "cfm2023",
    "unicon2020"
];
const UDA_DOMAIN: &str = "reg.unicycling-software.com/en";
pub const DATA_FOLDER: &str = "data";

fn build_client() -> Client {
    reqwest::ClientBuilder::new()
        .cookie_store(true)
        .build()
        .unwrap()
}

pub async fn download_data() -> Result<Vec<[String; 2]>, ()> {
    let credentials = Credentials::load_credentials()
        .or_else(|error| {
            eprintln!("Can't download data because no credential: {error}");
            Err(())
        })?;

    let mut downloaded_conventions = vec![];
    for convention_tag in CONVENTIONS_TAG {
        match create_folder_for_convention(&convention_tag) {
            Ok(_) => {}
            Err(_) => { continue; }
        }

        let client = build_client();
        let download_result = download_data_for_convention(&client, &credentials, convention_tag).await;
        if download_result.is_ok() {
            let convention_name = download_result.unwrap();
            println!("Convention has been successfully downloaded [convention: {}]", convention_name);
            downloaded_conventions.push([String::from(convention_tag), convention_name]);
        } else {
            let errors = download_result.unwrap_err();
            eprintln!("Errors encountered while downloading convention data [convention: {}]", convention_tag);
            for error in errors {
                eprintln!("{}", error);
            }
        }
    }

    Ok(downloaded_conventions)
}

fn create_folder_for_convention(convention_tag: &str) -> Result<(), ()> {
    create_dir_all(format!("{DATA_FOLDER}/{}", convention_tag)).or_else(
        |error| {
            eprintln!("Can't download data because can't create folder [convention: {}]", convention_tag);
            eprintln!("{error}");
            Err(())
        }
    )
}

async fn download_data_for_convention(client: &Client, credentials: &Credentials, convention: &str)
                                      -> Result<String, Vec<Error>> {
    let base_url = format!("https://{convention}.{UDA_DOMAIN}");
    let (convention_name, authenticity_token) = get_convention_name_and_authenticity_token(client, &base_url).await.or_else(|error| Err(vec![error]))?;
    login(client, &base_url, &authenticity_token, credentials).await.or_else(|error| Err(vec![error]))?;
    let results_future = export_results(&client, &base_url, convention);
    let registrants_futures = export_registrants(&client, &base_url, convention);

    let download_results = join!(results_future, registrants_futures);
    let mut errors = vec![];
    match download_results.0 {
        Ok(_) => {}
        Err(error) => { errors.push(error) }
    };
    match download_results.1 {
        Ok(_) => {}
        Err(error) => { errors.push(error) }
    };

    if errors.is_empty() {
        Ok(String::from(convention_name))
    } else {
        Err(errors)
    }
}

async fn get_convention_name_and_authenticity_token(client: &Client, base_url: &str)
                                                    -> Result<(String, String), Error> {
    let url = format!("{}/users/sign_in", base_url);
    let response = client.get(url)
        .send()
        .await
        .or_else(|error| Err(Error::new(ErrorKind::Other, format!("Can't get convention name or authenticity token: {error}"))))?;


    let body = response
        .text()
        .await
        .or_else(|error| Err(Error::new(ErrorKind::Other, format!("Can't get convention name or authenticity token: {error}"))))?;

    let document = Html::parse_document(&body);
    let convention_name = match get_convention_name_from_html(&document) {
        Ok(name) => { String::from(name) }
        Err(error) => { return Err(Error::new(ErrorKind::Other, format!("Can't get convention name: {error}"))); }
    };
    let authenticity_token = match get_authenticity_token_from_html(&document) {
        Ok(token) => { String::from(token) }
        Err(error) => { return Err(Error::new(ErrorKind::Other, format!("Can't get authenticity token: {error}"))); }
    };

    Ok((convention_name, authenticity_token))
}

fn get_authenticity_token_from_html(document: &Html) -> Result<&str, Error> {
    let token_selector = Selector::parse(r#"input[name="authenticity_token"]"#).unwrap();
    match document.select(&token_selector).next() {
        None => Err(Error::new(ErrorKind::NotFound, "Authenticity token not found")),
        Some(element) => {
            let authenticity_token = element
                .value()
                .attr("value")
                .unwrap();
            Ok(authenticity_token)
        }
    }
}

fn get_convention_name_from_html(document: &Html) -> Result<&str, Error> {
    let title_selector = Selector::parse(".title").unwrap();
    match document.select(&title_selector).next() {
        None => { Err(Error::new(ErrorKind::NotFound, "Convention name not found")) }
        Some(title_element) => {
            match title_element.text().next() {
                None => { Err(Error::new(ErrorKind::NotFound, "Convention name not found")) }
                Some(title) => { Ok(title) }
            }
        }
    }
}

async fn login(client: &Client, base_url: &str, authenticity_token: &str, credentials: &Credentials)
               -> Result<(), Error> {
    let url = format!("{}/users/sign_in", base_url);
    let params = [
        ("user[email]", credentials.username().as_str()),
        ("user[password]", credentials.password().as_str()),
        ("authenticity_token", authenticity_token),
        ("utf8", "✓"),
    ];
    let response = client.post(url)
        .form(&params)
        .send()
        .await
        .or_else(|error| Err(Error::new(ErrorKind::PermissionDenied, format!("Failed to authenticate: {error}"))))?;

    let status = response.status();
    let text = response.text().await
        .or_else(|error| Err(Error::new(ErrorKind::PermissionDenied, format!("Failed to authenticate: {error}"))))?;
    if text.contains("Signed in successfully")
        || text.contains("You are already signed in") {
        Ok(())
    } else {
        println!("Failed to authenticate: {status}");
        Err(Error::new(ErrorKind::PermissionDenied, format!("Failed to authenticate: {status}")))
    }
}

async fn export_results(client: &Client, base_url: &str, convention: &str) -> Result<(), Error> {
    let url = format!("{base_url}/export/results.xls");
    download_file(client, &url, &format!("{DATA_FOLDER}/{convention}/results.xls")).await
}

async fn export_registrants(client: &Client, base_url: &str, convention: &str) -> Result<(), Error> {
    let url = format!("{base_url}/export/download_registrants.xls");
    download_file(client, &url, &format!("{DATA_FOLDER}/{convention}/registrants.csv")).await
}

async fn download_file(client: &Client, url: &str, filepath: &str) -> Result<(), Error> {
    let response = match client.get(url)
        .send()
        .await {
        Ok(response) => { response }
        Err(error) => { return Err(Error::new(ErrorKind::Other, error)); }
    };
    let status = response.status();
    let body = response
        .bytes()
        .await
        .or_else(|error| Err(Error::new(ErrorKind::Other, error)))?;

    if status == StatusCode::OK
        && is_authorized(body.to_vec()) {
        let mut file = File::create(filepath).unwrap();
        return file.write_all(body.as_ref());
    }

    let error_message = format!("Can't download file [url:{url}, status:{status}]");
    Err(Error::new(ErrorKind::PermissionDenied, error_message))
}

fn is_authorized(body: Vec<u8>) -> bool {
    let body = String::from_utf8(body);
    match body {
        Ok(text) => { !text.contains("You are not authorized to perform this action") }
        Err(_) => { true }
    }
}