use std::collections::HashSet;

use glob::glob;
use serde_json::{Map, Value};

use crate::versions::v1::{extdata::{get_extdata_dir, v1_get_extdata_info}, save::{get_save_dir, v1_get_save_info}};


pub fn v1_get_title_info(id: u64) -> Map<String, Value> {
    let mut out: Map<String, Value> = Map::new();
    out.insert("id".to_string(), Value::Number(id.into()).into());
    out.insert("save".to_string(), v1_get_save_info(id).into());
    out.insert("extdata".to_string(), v1_get_extdata_info(id).into());

    out
}

pub fn v1_get_title_ids() -> Vec<u64> {
    let save_dir = format!("{}/*", get_save_dir().to_str().expect("Failed to get save directory"));
    let extdata_dir = format!("{}/*", get_extdata_dir().to_str().expect("Failed to get extdata directory"));

    glob(&save_dir)
        .unwrap()
        .chain(
            glob(&extdata_dir)
            .unwrap()
        )
        .into_iter()
        .map(|path| {
            u64::from_str_radix(
                path.expect("Failed to get path")
                    .file_name()
                    .expect("Failed to get file name")
                    .to_str()
                    .unwrap(),
                16
            ).expect("Failed to convert name to u64")
        })
        .collect::<HashSet<u64>>()
        .into_iter()
        .collect::<Vec<u64>>()
}

#[get("/v1/title/<id>")]
pub fn v1_get_title(id: u64) -> Vec<u8> {
    serde_json::to_vec(&v1_get_title_info(id)).expect("Failed to serialize JSON")
}

#[get("/v1/title")]
pub fn v1_get_titles() -> Vec<u8> {
    serde_json::to_vec(
        &v1_get_title_ids().into_iter().map(|id| {
            Value::Object(v1_get_title_info(id))
        }).collect::<Vec<Value>>()
    ).expect("Failed to serialize JSON")
}