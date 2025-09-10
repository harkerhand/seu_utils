use crate::WantCourse;
use reqwest::{Client, Error};
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;
use urlencoding::encode;

/// 选择一个单独课程
pub async fn select_course(
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

    let course_select_url = "https://newxk.urp.seu.edu.cn/xsxk/elective/clazz/add";

    let res = client
        .post(course_select_url)
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
    let body: Value = serde_json::from_str(&body).unwrap();
    println!("Message: {}", body["msg"].as_str().unwrap());

    Ok(())
}

/// 获取课程列表
pub async fn get_course_list(
    class_type: &str,
    client: &Client,
    token: &str,
    batch_id: &str,
    classes_json: &PathBuf,
) -> Result<Value, Error> {
    let request_body = serde_json::json!({
        "teachingClassType": class_type,
        "pageNumber": 1,
        "pageSize": 99,
        "orderBy": "",
        "campus": "1",
    });

    let course_list_url = "https://newxk.urp.seu.edu.cn/xsxk/elective/clazz/list";
    let res = client
        .post(course_list_url)
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
    println!("Content: {}", body);

    let mut file = fs::File::create(classes_json).unwrap();
    file.write_all(body.as_bytes()).unwrap();

    Ok(serde_json::from_str(&body).unwrap())
}

/// 生成选课列表
pub async fn gene_wish_list(
    client: &Client,
    token: &str,
    batch_id: &str,
    classes_json: &PathBuf,
    choose_json: &PathBuf,
) -> Result<(), Error> {
    println!("Generating wish list...");
    let courses = get_course_list("TJKC", client, token, batch_id, classes_json).await?;
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

    let json_data = serde_json::to_string_pretty(&want_courses).unwrap();
    let mut file = fs::File::create(choose_json).unwrap();
    file.write_all(json_data.as_bytes()).unwrap();

    Ok(())
}

/// 选课
pub async fn choose_courses(
    client: &Client,
    token: &str,
    batch_id: &str,
    choose_json: &PathBuf,
) -> Result<(), Error> {
    println!("Choosing courses...");
    let want_courses: Vec<WantCourse> =
        serde_json::from_str(&fs::read_to_string(choose_json).unwrap()).unwrap();

    let mut i = 0;
    loop {
        for course in &want_courses {
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
    }
}
