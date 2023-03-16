use std::{env, fmt::format, fs};

use serde_json::{Number, Value};

use crate::usertypes::{self, user_types};

pub fn new(coll_type: String) {
    let app_dir = env::var("LAPKA_FILES").unwrap();
    let t = usertypes::by_name(coll_type);
    fs::write(format!("{}/collections/{}", app_dir, t.name), "").unwrap();
}

pub fn insert(coll_type: String, data: String) {
    let t = usertypes::by_name(coll_type);
    let jdata: Value = serde_json::from_str(&data).unwrap();
    let rows_bytes: Vec<Vec<u8>> = t
        .fields
        .into_iter()
        .map(|f| {
            let field_val = &jdata[&f.name];
            match f.type_name.as_str() {
                "int" => int_bytes(&field_val),
                &_ => panic!("unknown type"),
            }
        })
        .collect();
    let result: Vec<u8> = rows_bytes.into_iter().flatten().collect();
    let app_dir = env::var("LAPKA_FILES").unwrap();
    fs::write(format!("{}/collections/{}", app_dir, t.name), result).unwrap();
}

fn int_bytes(value: &Value) -> Vec<u8> {
    let x = match value {
        Value::Number(n) => n,
        _ => panic!(),
    };
    x.as_u64().unwrap().to_ne_bytes().to_vec()
}

#[test]
fn insert_test() {
    env::set_var("LAPKA_FILES", "test_dir3");
    insert(
        "user".to_string(),
        r#"{
            "id": 3
            }"#
        .to_string(),
    );
}
