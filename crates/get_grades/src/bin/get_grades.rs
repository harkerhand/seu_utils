use get_grades::Course;
use reqwest::Client;
use serde_json::Value;
use std::fs;
use tokio;

#[tokio::main]
async fn main() {
    let cookio: String = fs::read_to_string("resource/grades_cookie.txt").unwrap();
    let client = Client::new();
    let grades_url = "https://ehall.seu.edu.cn/jwapp/sys/cjcx/modules/cjcx/xscjcx.do";
    let response =  client.post(grades_url)
        .header("Cookie", cookio)
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
    fs::write("resource/raw_grades.json", formatted_json).unwrap();
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
    fs::write(
        "resource/grades.json",
        serde_json::to_string_pretty(&grades).unwrap(),
    )
    .unwrap();
}
