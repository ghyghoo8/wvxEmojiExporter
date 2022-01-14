/*
 * @Author: your name
 * @Date: 2021-12-23 20:15:16
 * @LastEditTime: 2022-01-14 16:17:51
 * @LastEditors: Please set LastEditors
 * @Description: 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 * @FilePath: /scaffold-demo/tauri-app/tauri-app/src-tauri/src/main.rs
 */
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::json;
use dirs_next::home_dir;

use walkdir::{DirEntry, WalkDir};


use rustc_serialize::base64::{ ToBase64, MIME };
use rustc_serialize::hex::{ToHex};
use regex::Regex;
use std::io::Read;
use std::string::String;



#[derive(Serialize, Deserialize)]
struct JsonData {
  CurrentProfile: String,
}

#[derive(Serialize, Deserialize)]
struct ImageItem {
  base64: String,
  name: String,
  ty: String
}



fn main() {
  tauri::Builder::default()
    // This is where you pass in your commands
    .invoke_handler(tauri::generate_handler![my_custom_command, search_folder_by_path])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn search_folder_by_path(folder_path: String, export_path: String) -> Result<bool, String> {
  println!("{}","start===>");
  // let mut res_list: Vec<ImageItem> = vec![];
  let walker = WalkDir::new(folder_path).into_iter();
  // 要导出的文件夹路径
  let export_path_value = Path::new(&export_path);
  // 创建文件夹
  let is_ok = fs::create_dir_all(export_path_value);

  // 过滤 文件夹 和 隐藏文件
  for entry in walker.filter_entry(|e| !is_hidden(e)) {
    let entry = entry.unwrap();
    if !entry.file_type().is_dir() {
      // println!("{}", entry.path().display());
      let (img_code, file_type) = to_base64(&entry.path().display().to_string());
      if !file_type.is_empty() {
        let file_name = entry.path().file_name().unwrap();
        fs::copy(entry.path(), export_path_value.join(file_name));

        // 获得文件类型，文件名及 base64值
        // let image_item = ImageItem {
        //   base64: img_code,
        //   name: format!("{:?}", file_name),
        //   ty: file_type
        // };
        // res_list.push(image_item);
      }
    }
  }
  // 将搜集到的文件列表，存到指定path
  // for fileItem in res_list {

  // }

  println!("{}","end===>");

  Ok(true)
}

#[tauri::command]
fn my_custom_command(invoke_message: String) -> String {

  let home_dir = home_dir().unwrap();

  let json_path = home_dir.join(invoke_message).as_path().display().to_string();
  let mut value = "".to_owned();
  if path_exists(&json_path) {
    let json_contents = fs::read_to_string(&json_path).expect("Something went wrong reading the file");
    let v: JsonData = serde_json::from_str(&json_contents).unwrap();
    // 获取当前的位置
    value = v.CurrentProfile;
  } 
  let json_path_data = json!({
      "path": json_path,
      "code": value
  });
  return json_path_data.to_string();
}

// 获取 图片后缀
pub fn get_file_type(hex: &str) -> &str {
  if Regex::new(r"^ffd8ffe0").unwrap().is_match(hex) { 
      return "jpeg" 
  } else if Regex::new(r"^89504e47").unwrap().is_match(hex) {  
      return "png" 
  } else if Regex::new(r"^47494638").unwrap().is_match(hex) { 
      return "gif"
  } 
  // panic!("invalid file type")
  return "";
}

// 转换为base64
pub fn to_base64(path: &str) -> (String, String) {
  let mut file = fs::File::open(path).unwrap();
  let mut vec = Vec::new();
  let _ = file.read_to_end(&mut vec);
  let base64 = vec.to_base64(MIME);
  let hex = vec.to_hex();
  let file_type = get_file_type(&hex);
  let image_code = format!("data:image/{};base64,{}", file_type, base64.replace("\r\n", ""));
  return (image_code, file_type.to_owned());
}



// 判断 是否存在
pub fn path_exists(path: &str) -> bool {
  fs::metadata(path).is_ok()
}

// 判断是否是 隐藏文件 
pub fn is_hidden(entry: &DirEntry) -> bool {
  entry.file_name()
       .to_str()
       .map(|s| s.starts_with("."))
       .unwrap_or(false)
}

