use choose_classes::utils::{choose_courses, gene_wish_list};
use choose_classes::Config;
use clap::Parser;
use reqwest::Client;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version, name = "get_tokens")]
#[command(version, about, long_about = None)]
struct Cli {
    /// 配置文件路径 IN_NEED
    #[clap(long, default_value = "resource/config.yaml")]
    config_yaml: PathBuf,
    /// 全部课程路径
    #[clap(long, default_value = "resource/classes.json")]
    classes_json: PathBuf,
    /// 选择课程路径
    #[clap(long, default_value = "resource/choose.json")]
    choose_json: PathBuf,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config: Config =
        serde_yaml::from_str(&fs::read_to_string(&cli.config_yaml).unwrap()).unwrap();
    let token = &config.token;
    let batch_id = &config.batch_id;
    println!("Token: {}", token);
    println!("Batch ID: {}", batch_id);

    let client = Client::new();

    print!("是否需要重新生成志愿表？(y/n)：");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    gene_wish_list(
        &client,
        token,
        batch_id,
        &cli.classes_json,
        &cli.choose_json,
        input.eq_ignore_ascii_case("y"),
    )
    .await
    .unwrap();

    print!("按下回车开始选课...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    choose_courses(&client, token, batch_id, &cli.choose_json)
        .await
        .unwrap();
}
