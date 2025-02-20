use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::time::Duration;
use tokio::time::sleep;
use urlencoding::encode;

const COURSE_LIST_URL: &str = "https://newxk.urp.seu.edu.cn/xsxk/elective/clazz/list";
const COURSE_SELECT_URL: &str = "https://newxk.urp.seu.edu.cn/xsxk/elective/clazz/add";

#[derive(Serialize, Deserialize, Debug)]
struct Course {
    tc_list: Vec<TcList>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TcList {
    jxbid: String,
    secret_val: String,
    kcm: String,
    sksj: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    token: String,
    batch_id: String,
}

async fn get_course_list(
    class_type: &str,
    client: &Client,
    token: &str,
    batch_id: &str,
) -> Result<Value, Error> {
    let request_body = serde_json::json!({
        "teachingClassType": class_type,
        "pageNumber": 1,
        "pageSize": 20,
        "orderBy": "",
        "campus": "1",
    });

    let res = client
        .post(COURSE_LIST_URL)
        .header("Authorization", token)
        .header("Content-Type", "application/json;charset=UTF-8")
        .header("Accept", "application/json, text/plain, */*")
        .header("Origin", "https://newxk.urp.seu.edu.cn")
        .header(
            "Referer",
            format!(
                "https://newxk.urp.seu.edu.cn/xsxk/elective/grablessons?batchId={}&token={}",
                batch_id, token
            ),
        )
        .json(&request_body)
        .send()
        .await?;

    let status_code = res.status();
    println!("Status Code: {}", status_code);

    let body = res.text().await?;
    // println!("Content: {}", body);

    let file_path = "resource/jsons/classes.json";
    let mut file = fs::File::create(file_path).unwrap();
    file.write_all(body.as_bytes()).unwrap();

    Ok(serde_json::from_str(&body).unwrap())
}

async fn select_course(
    course_type: &str,
    course_id: &str,
    course_secret: &str,
    client: &Client,
    token: &str,
    batch_id: &str,
) -> Result<(), Error> {
    let form_data = format!(
        "clazzType={}&clazzId={}&secretVal={}",
        course_type,
        course_id,
        encode(course_secret)
    );

    let res = client
        .post(COURSE_SELECT_URL)
        .header("Authorization", token)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json, text/plain, */*")
        .header("Origin", "https://newxk.urp.seu.edu.cn")
        .header(
            "Referer",
            format!(
                "https://newxk.urp.seu.edu.cn/xsxk/elective/grablessons?batchId={}&token={}",
                batch_id, token
            ),
        )
        .body(form_data)
        .send()
        .await?;

    let status_code = res.status();
    println!("Status Code: {}", status_code);

    let body = res.text().await?;
    println!("Content: {}", body);

    Ok(())
}

async fn gene_wish_list(client: &Client, token: &str, batch_id: &str) -> Result<(), Error> {
    let courses = get_course_list("TJKC", client, token, batch_id).await?;
    let data = courses["data"]["rows"].as_array().unwrap();

    let mut want_courses = Vec::new();

    for course in data {
        let tclist = course["tcList"].as_array().unwrap();
        for clss in tclist {
            let jxb_id = clss["JXBID"].as_str().unwrap();
            let secret = clss["secretVal"].as_str().unwrap();
            let name = clss["KCM"].as_str().unwrap();
            let unknow = Value::String("unknow".to_string());
            let skjs = clss.get("SKJS").unwrap_or(&unknow).as_str().unwrap();

            println!(
                "Do you want to select the course {}({})? (y/n): ",
                name, skjs
            );
            let mut user_input = String::new();
            std::io::stdin().read_line(&mut user_input).unwrap();

            if user_input.trim().to_lowercase() != "y" {
                continue;
            }

            want_courses.push(WantCourse {
                name: format!("{}({})", name, skjs),
                course_type: "TJKC".to_string(),
                course_id: jxb_id.to_string(),
                course_secret: secret.to_string(),
            });
        }
    }

    let file_path = "resource/jsons/choose.json";
    let json_data = serde_json::to_string_pretty(&want_courses).unwrap();
    let mut file = fs::File::create(file_path).unwrap();
    file.write_all(json_data.as_bytes()).unwrap();

    Ok(())
}

async fn choose_courses(client: &Client, token: &str, batch_id: &str) -> Result<(), Error> {
    let want_courses: Vec<WantCourse> =
        serde_json::from_str(&fs::read_to_string("resource/jsons/choose.json").unwrap()).unwrap();

    let mut i = 0;
    for course in want_courses {
        select_course(
            &course.course_type,
            &course.course_id,
            &course.course_secret,
            client,
            token,
            batch_id,
        )
        .await?;
        println!("Selected course: {}", course.name);
        i += 1;

        if i % 3 == 0 {
            sleep(Duration::from_secs(1)).await;
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct WantCourse {
    name: String,
    course_type: String,
    course_id: String,
    course_secret: String,
}

#[tokio::main]
async fn main() {
    let config: Config =
        serde_yaml::from_str(&fs::read_to_string("resource/jsons/config.yaml").unwrap()).unwrap();
    let token = &config.token;
    let batch_id = &config.batch_id;

    let client = Client::new();

    gene_wish_list(&client, token, batch_id).await.unwrap();
    // choose_courses(&client, token, batch_id).await.unwrap();
}
