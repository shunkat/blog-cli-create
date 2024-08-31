use clap::{Arg, Command};
use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use chrono::{Local};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use ulid::Ulid;

#[derive(Serialize, Deserialize, Debug)]
struct DbEntry {
    category: String,
    slug: String,
}

fn main() {
    // 1. Clapによるコマンドライン引数処理
    let matches = Command::new("File Creator")
        .arg(Arg::new("slug").long("slug").help("File slug"))
        .arg(Arg::new("category").long("category").help("File category"))
        .get_matches();

    // 2. Dialoguerによる対話型入力 (引数がない場合)
    let slug = matches
        .get_one::<String>("slug")
        .cloned()
        .unwrap_or_else(|| Input::<String>::new().with_prompt("Slug:").interact().unwrap());

    let categories = vec!["book", "flutter", "entrance", "ai", "other"];
    let category_index = Select::new()
        .with_prompt("Category:")
        .items(&categories)
        .default(0)
        .interact()
        .unwrap();
    let category = categories[category_index].to_string();

    // 3. ファイル作成
    let contents_dir = Path::new("./origin/contents");
    let file_path = contents_dir.join(&category).join(format!("{}.md", &slug));

    if let Some(parent) = file_path.parent() {
        create_dir_all(parent).expect("Failed to create directory");
    }
    let mut file = File::create(&file_path).expect("Failed to create file");

    // 現在時刻を取得
    let now = Local::now();  // chrono::Localを使用

    // UUIDを生成
    let ulid = Ulid::new();
    let id = ulid.to_string();

    // フロントマターの追加
    let front_matter = format!(
        r#"+++
id = "{}"
title = ""
date = "{}"
draft = true
summary = ""
emoji = ""
tags = []
category = "{}"
slug = "{}"
+++
"#,
        id, 
        now.format("%Y-%m-%dT%H:%M:%S%:z"),
        category,
        slug,
    );

    file.write_all(front_matter.as_bytes())
        .expect("Failed to write front matter");
}
