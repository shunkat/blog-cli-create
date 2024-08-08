use clap::{Arg, Command};
use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::Path;

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

    let categories = vec!["Kotlin", "Flutter", "entrance", "gadget", "others"];
    let category_index = Select::new()
        .with_prompt("Category:")
        .items(&categories)
        .default(0)
        .interact()
        .unwrap();
    let category = categories[category_index].to_string();

    // 3. ファイル作成
    let file_path = Path::new(&category).join(&slug);
    if let Some(parent) = file_path.parent() {
        create_dir_all(parent).expect("Failed to create directory");
    }
    File::create(&file_path).expect("Failed to create file");

    // 4. DB (JSON) 更新
    let new_entry = DbEntry { category, slug };
    let mut data: Value = serde_json::from_str(
        &std::fs::read_to_string("db.json").unwrap_or_else(|_| "[]".to_string()),
    )
    .unwrap_or_default();
    if let Some(arr) = data.as_array_mut() {
        arr.push(json!(new_entry));
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("db.json")
        .expect("Failed to open db.json");
    file.write_all(serde_json::to_string_pretty(&data).unwrap().as_bytes())
        .expect("Failed to write to db.json");
}
