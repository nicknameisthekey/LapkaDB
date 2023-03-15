use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{env, fmt, fs};

#[derive(Serialize, Deserialize)]
pub struct UserType {
    pub name: String,
    pub fields: Vec<UserTypeField>,
}

#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct UserTypeField {
    pub name: String,
    pub type_name: String,
    pub nullable: bool,
    pub key: bool,
}

pub fn user_types() -> Vec<UserType> {
    let app_dir = env::var("LAPKA_FILES").unwrap();
    return fs::read_dir(format!("{}/types", app_dir))
        .unwrap()
        .map(|entry| {
            let file_text = fs::read_to_string(entry.unwrap().path());
            return serde_json::from_str(&file_text.unwrap()).unwrap();
        })
        .collect();
}

pub fn by_name(name: String) -> UserType {
    let types = user_types();
    types.into_iter().find(|t| t.name == name).unwrap()
}

pub fn add_user_type(json: String) {
    let t: UserType = serde_json::from_str(&json).unwrap();
    let app_dir = env::var("LAPKA_FILES").unwrap();
    fs::write(format!("{}/types/{}.json", app_dir, &t.name), json).unwrap();
}


mod user_type_tests {
    use std::env;

    use crate::usertypes::*;

    #[test]
    fn read_types_from_files() {
        env::set_var("LAPKA_FILES", "test_dir");
        let types = user_types();
        assert_eq!(1, types.len());
        types.iter().for_each(|t| match t.name.as_str() {
            "menu" => {
                assert_eq!(3, t.fields.len());
                assert!(t.fields.iter().any(|item| {
                    *item
                        == UserTypeField {
                            name: "dish_name".to_string(),
                            type_name: "string".to_string(),
                            nullable: false,
                            key: true,
                        }
                }));
                assert!(t.fields.iter().any(|item| {
                    *item
                        == UserTypeField {
                            name: "price".to_string(),
                            type_name: "int".to_string(),
                            nullable: false,
                            key: false,
                        }
                }));
                assert!(t.fields.iter().any(|item| {
                    *item
                        == UserTypeField {
                            name: "description".to_string(),
                            type_name: "string".to_string(),
                            nullable: true,
                            key: false,
                        }
                }));
            }
            &_ => {
                assert!(false)
            }
        });
    }

    #[test]
    fn add_type_creates_file() {
        env::set_var("LAPKA_FILES", "test_dir2");
        add_user_type(
            r#"{
            "name": "user",
            "fields": [
                {
                    "name": "username",
                    "type_name": "string",
                    "nullable": false,
                    "key": true
                }
            ]
        }"#
            .to_string(),
        );

        let types = user_types();
        assert_eq!(1, types.len());
        assert_eq!(1, types[0].fields.len());

        assert_eq!(
            UserTypeField {
                name: "username".to_string(),
                type_name: "string".to_string(),
                nullable: false,
                key: true,
            },
            types[0].fields[0]
        );

        fs::remove_file("test_dir2/types/user.json").unwrap();
    }
}
