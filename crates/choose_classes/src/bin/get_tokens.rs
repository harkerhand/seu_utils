use base64::Engine;
use cfg_if::cfg_if;
use choose_classes::Config;
use clap::Parser;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tokio;

#[derive(Parser)]
#[clap(version, name = "get_tokens")]
#[command(version, about, long_about = None)]
struct Cli {
    /// 配置文件路径 IN_NEED
    #[clap(long, default_value = "resource/config.yaml")]
    config_yaml: PathBuf,
    /// 临时验证码路径
    #[clap(long, default_value = "resource/captcha.png")]
    captcha_png: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginData {
    loginname: String,
    password: String,
    captcha: String,
    uuid: String,
}

fn decode_and_show_image(
    encoded_image: &str,
    captcha_png: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let img_data =
        base64::engine::general_purpose::STANDARD.decode(&encoded_image.as_bytes().to_vec())?;
    let img = image::load_from_memory(&img_data)?;
    img.save(&captcha_png)?;

    let current_dir = std::env::current_dir().unwrap();
    let captcha_png = current_dir.join(captcha_png);
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            Command::new("cmd")
                .arg("/C")
                .arg(format!("start {}", captcha_png.to_str().unwrap()))
                .spawn()?;
        } else if #[cfg(target_os = "macos")] {
            Command::new("open").arg(captcha_png).spawn()?;
        } else {
            Command::new("xdg-open").arg(captcha_png).spawn()?;
        }
    }

    Ok(())
}

async fn get_login_data(cli: &Cli) -> Result<LoginData, Error> {
    let config: Config =
        serde_yaml::from_str(&fs::read_to_string(&cli.config_yaml).unwrap()).unwrap();
    let loginname = config.loginname;
    let password = config.password;
    let captcha_url = "https://newxk.urp.seu.edu.cn/xsxk/auth/captcha";
    let client = Client::new();
    let response = client
        .post(captcha_url)
        .header("Content-Length", "0")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
        .send()
        .await?;
    let response: Value = serde_json::from_str(&response.text().await?).unwrap();
    println!("Response: {}", response["msg"]);
    let uuid = response["data"]["uuid"].as_str().unwrap();
    let captcha = response["data"]["captcha"].as_str().unwrap();
    let encoded_image = captcha.split(",").nth(1).unwrap();
    decode_and_show_image(encoded_image, &cli.captcha_png).unwrap();

    println!("Please input the captcha:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let captcha = input.trim();
    Ok(LoginData {
        loginname,
        password,
        captcha: captcha.to_string(),
        uuid: uuid.to_string(),
    })
}

async fn get_token(data: &LoginData) -> Result<String, Error> {
    let auth_url = "https://newxk.urp.seu.edu.cn/xsxk/auth/login";
    let client = Client::new();

    let response = client
        .post(auth_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json, text/plain, */*")
        .header("Origin", "https://newxk.urp.seu.edu.cn")
        .header(
            "Referer",
            "https://newxk.urp.seu.edu.cn/xsxk/profile/index.html",
        )
        .form(&data)
        .send()
        .await?;

    let response_text = &response.text().await?;
    println!("Token response: {}", response_text);
    let response: Value = serde_json::from_str(response_text).unwrap();
    Ok(response["data"]["token"].as_str().unwrap().to_string())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut config: Config =
        serde_yaml::from_str(&fs::read_to_string(&cli.config_yaml).unwrap()).unwrap();
    let batch_id = &config.batch_id;
    let login_data = get_login_data(&cli).await.unwrap();
    let token = get_token(&login_data).await.unwrap();
    println!("Token: {}", token);
    config.token = token.clone();
    fs::write(cli.config_yaml, serde_yaml::to_string(&config).unwrap()).unwrap();

    let elective_url = format!(
        "https://newxk.urp.seu.edu.cn/xsxk/elective/grablessons?batchId={}&token={}",
        batch_id, token
    );
    println!("Elective URL: {}", elective_url);
    let client = Client::new();
    client
        .get(&elective_url)
        .header("Content-Type", "application/json;charset=UTF-8")
        .header("Accept", "application/json, text/plain, */*")
        .header("Origin", "https://newxk.urp.seu.edu.cn")
        .header(
            "Referer",
            "https://newxk.urp.seu.edu.cn/xsxk/profile/index.html",
        )
        .send()
        .await
        .unwrap();
}
