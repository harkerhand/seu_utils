use clap::Parser;
use get_grades::Course;
use reqwest::Client;
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tokio;

#[derive(Parser)]
#[clap(version, name = "get_grades")]
#[command(version, about, long_about = None)]
struct Cli {
    /// cookie路径 IN_NEED
    #[clap(short, long, default_value = "resource/grades_cookie.txt")]
    cookie_txt: PathBuf,
    /// 保存导出成绩的json文件路径
    #[clap(short, long, default_value = "resource/grades.json")]
    grades_json: PathBuf,
    /// 保存原始成绩的json文件路径
    #[clap(short, long, default_value = "resource/raw_grades.json")]
    raw_grades_json: PathBuf,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let cookie = fs::read_to_string(cli.cookie_txt)
        .unwrap()
        .trim()
        .to_string();
    let client = Client::new();
    let grades_url = "https://ehall.seu.edu.cn/jwapp/sys/cjcx/modules/cjcx/xscjcx.do";
    let response =  client.post(grades_url)
        .header("Cookie", cookie)
        .header(
            "user-agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36 Edg/133.0.0.0"
        )
        .header("Content-Length", "0")
        .send().await.unwrap();
    if response.status() != 200 {
        panic!("Failed to get grades");
    }
    let text = response.text().await.unwrap();
    let formatted_json = {
        let parsed: Value = serde_json::from_str(&text).unwrap();
        serde_json::to_string_pretty(&parsed).unwrap()
    };
    let mut raw_grades_file = fs::File::create(&cli.raw_grades_json).unwrap();
    raw_grades_file
        .write_all(formatted_json.as_bytes())
        .unwrap();
    let response: Value = serde_json::from_str(&text).unwrap();

    let grades = response["datas"]["xscjcx"]["rows"].as_array().unwrap();
    let grades = grades
        .iter()
        .map(|grade| Course {
            name: grade["KCM"].as_str().unwrap().to_string(),
            class_type: grade["KCXZDM_DISPLAY"].as_str().unwrap().to_string(),
            score: grade["ZCJ"].as_str().unwrap().parse().unwrap(),
            credit: grade["XF"].as_str().unwrap().parse().unwrap(),
            term: grade["XNXQDM"].as_str().unwrap().into(),
        })
        .collect::<Vec<Course>>();
    let mut grades_file = fs::File::create(&cli.grades_json).unwrap();
    grades_file
        .write_all(serde_json::to_string_pretty(&grades).unwrap().as_bytes())
        .unwrap();
}
