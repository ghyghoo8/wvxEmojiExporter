use std::fs;
use std::io::Read;
use regex::Regex;
use std::string::String;
use walkdir::{ DirEntry };
use rustc_serialize::base64::{ ToBase64, MIME };
use rustc_serialize::hex::{ToHex};

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