use std::path::{Path, PathBuf};

use glob::glob;
use rocket::http::Status;
use serde_json::{json, Map, Value};

pub fn get_save_dir() -> PathBuf {
    Path::new("./saves").to_path_buf()
}

fn get_save_path(id: u64) -> PathBuf {
    get_save_dir().join(format!("{:X}", id))
}

pub fn v1_get_save_info(id: u64) -> Vec<Map<String, Value>> {
    glob(&format!("{}/**/*", get_save_path(id).to_str().unwrap()))
        .expect("Invalid glob pattern")
        .filter_map(|entry|{
        let file = entry.expect("Error reading path");
        if !file.exists() || !file.is_file() {
            return None
        }

        Some(json!({
            "path": format!("/{}",
                file.iter()
                    .skip_while(|s| *s != format!("{:X}", id).as_str())
                    .skip(1)
                    .collect::<PathBuf>()
                    .to_string_lossy()
            ),
            "hash": sha256::TrySha256Digest::digest(file.clone()).expect("Failed to get file hash"),
            "size": file.metadata().expect("Failed to get file metadata").len()
        }).as_object().unwrap().clone())
    }).collect()
}

#[get("/v1/save")]
pub fn v1_get_saves() -> Vec<u8> {
    serde_json::to_vec(&glob(&format!("{}/*", Path::new("./saves").to_path_buf().to_str().unwrap()))
        .expect("Invalid glob pattern")
        .filter_map(|entry| {
        let path = entry.unwrap();
        if !path.is_dir() {
            return None
        }

        let name = path.file_name().expect("Failed to get folder name").to_str().unwrap();
        Some((name.to_string(), v1_get_save_info(u64::from_str_radix(name, 16).expect("Failed to convert name to u64")).into()))
    }).collect::<Map<String, Value>>()).expect("Failed to serialize JSON")
}

#[get("/v1/save/<id>/<file_path..>")]
pub fn v1_get_save_file(id: u64, file_path: PathBuf) -> Result<Vec<u8>, Status> {
    let folder_path = get_save_path(id);

    if file_path.as_os_str().is_empty() {
        return Ok(serde_json::to_vec(&v1_get_save_info(id)).expect("Failed to serialize JSON"));
    }

    let target_path = folder_path.join(file_path);
    if !target_path.exists() || target_path.is_dir() {
        return Err(Status::NotFound)
    }

    Ok(std::fs::read(target_path).expect("Failed to read file"))
}

#[delete("/v1/save/<id>/<save_file_path..>")]
pub fn v1_delete_save(id: u64, save_file_path: PathBuf) -> Status {
    let path = get_save_path(id).join(save_file_path);
    
    match path.is_file() {
    true => std::fs::remove_file(path),
    false => std::fs::remove_dir_all(path)
    }.map_or(Status::Forbidden, |_| Status::NoContent)
}

#[put("/v1/save/<id>/<save_file_path..>", data = "<save_data>")]
pub fn v1_put_save(id: u64, save_data: &[u8], save_file_path: PathBuf) -> Status {
    let file_path = format!("{}/{}", get_save_path(id).as_path().to_str().unwrap(), save_file_path.to_str().unwrap());
    let path = std::path::Path::new(file_path.as_str());
    let parent = path.parent().unwrap();

    if !std::fs::exists(parent).unwrap() {
        std::fs::create_dir_all(parent).unwrap();
    }

    std::fs::write(file_path, save_data).unwrap();
    Status::NoContent
}