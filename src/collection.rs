use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{Read, Write},
};

use serde_json::{Number, Value};

use crate::{
    page::{self, Page},
    usertypes::{self, user_types},
};

/*
collection header todo

- посчитать размер записи
- взять конец файла - размер страницы
- в хедере страницы посмотреть сколько доступного места
- если места нет - создаем новую страницу
- добавляем данные по оффсету, меняем хедер
- пишем в файл
*/

pub fn new(coll_type: String) {
    let t = usertypes::by_name(coll_type);
    let page = Page::new();
    let bytes = page.to_bytes();
    let mut f = File::create(coll_filename(&t.name)).unwrap();
    f.write(&bytes);
}

fn coll_filename(coll_name: &str) -> String {
    let app_dir = env::var("LAPKA_FILES").unwrap();
    format!("{}/collections/{}", app_dir, coll_name)
}

pub fn insert(coll_type: String, data: String) {
    let t = usertypes::by_name(coll_type);
    let jdata: Value = serde_json::from_str(&data).unwrap();
    let rows_bytes: Vec<Vec<u8>> = t
        .fields
        .into_iter()
        .map(|f| {
            let field_val = &jdata[&f.name];
            to_bytes(field_val, &f.type_name).unwrap()
        })
        .collect();
    let result: Vec<u8> = rows_bytes.into_iter().flatten().collect();

    let mut f = File::open(coll_filename(&t.name)).unwrap();
    let mut file_bytes = Vec::new();
    f.read_to_end(&mut file_bytes).unwrap();
    let mut page = Page::from_bytes(&file_bytes);
    let file_bytes = page.append(result).to_bytes();
    let mut f = File::create(coll_filename(&t.name)).unwrap(); //todo   hack
    f.write(&file_bytes).unwrap();
}

#[derive(Debug, Clone)]
struct TypeMismatchError;

fn to_bytes(value: &Value, t: &str) -> Result<Vec<u8>, TypeMismatchError> {
    match t {
        "int" => match value {
            Value::Number(n) => Ok(int_bytes(n)),
            _ => Err(TypeMismatchError),
        },
        "string" => match value {
            Value::String(s) => Ok(string_bytes(s)),
            _ => Err(TypeMismatchError),
        },
        &_ => Err(TypeMismatchError),
    }
}

fn string_bytes(value: &String) -> Vec<u8> {
    let str_bytes = value.as_bytes();
    let str_size = str_bytes.len().to_be_bytes();
    let mut result: Vec<u8> = Vec::new();
    result.extend(str_size);
    result.extend(str_bytes);
    result
}

fn int_bytes(value: &Number) -> Vec<u8> {
    value.as_u64().unwrap().to_be_bytes().to_vec()
}

#[test]
fn insert_test() {
    env::set_var("LAPKA_FILES", "test_dir3");
    new("user".to_string());
    insert(
        "user".to_string(),
        r#"{
            "id": 3,
            "name":"pepega",
            "role":"admin"
            }"#
        .to_string(),
    );
    insert(
        "user".to_string(),
        r#"{
            "id": 3,
            "name":"ded",
            "role":"user"
            }"#
        .to_string(),
    );
    insert(
        "user".to_string(),
        r#"{
            "id": 3,
            "name":"andreyka",
            "role":"bambooster"
            }"#
        .to_string(),
    );
}
