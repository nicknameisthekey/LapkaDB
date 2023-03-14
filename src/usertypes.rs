pub mod usertypes {
    use std::{env, fs};

    pub struct UserType {
        name: String,
        fields: Vec<UserTypeField>,
    }
    #[derive(PartialEq, PartialOrd, Debug)]
    struct UserTypeField {
        name: String,
        type_name: String,
        nullable: bool,
        layout_pos: u8,
    }

    impl UserType {
        fn from_file_string(file_string: String) -> UserType {
            let mut lines = file_string.split("\n");
            let header = lines.next().unwrap();
            let fields = lines
                .map(|line| {
                    let values: Vec<&str> = line.split(',').collect();
                    return UserTypeField {
                        name: values[0].to_string(),
                        type_name: values[1].to_string(),
                        nullable: values[2].chars().next().unwrap() == '1',
                        layout_pos: values[3].parse().unwrap(),
                    };
                })
                .collect();
            return UserType {
                name: header.to_string(),
                fields: fields,
            };
        }
    }

    fn read_files() -> Vec<UserType> {
        let app_dir = env::var("LAPKA_FILES").unwrap();
        return fs::read_dir(app_dir)
            .unwrap()
            .map(|entry| {
                let file_text = fs::read_to_string(entry.unwrap().path());
                return UserType::from_file_string(file_text.unwrap());
            })
            .collect();
    }

    #[test]
    fn read_type_definition_from_string() {
        let result =
            UserType::from_file_string("type_name\nname,string,0,0\nage,int,1,1".to_string());

        assert_eq!(2, result.fields.len());
        assert!(result.fields.iter().any(|item| {
            *item
                == UserTypeField {
                    name: "name".to_string(),
                    type_name: "string".to_string(),
                    nullable: false,
                    layout_pos: 0,
                }
        }));

        assert!(result.fields.iter().any(|item| {
            *item
                == UserTypeField {
                    name: "age".to_string(),
                    type_name: "int".to_string(),
                    nullable: true,
                    layout_pos: 1,
                }
        }));
    }
    
    #[test]
    fn read_type_definition_from_file() {
        env::set_var("LAPKA_FILES", "test_dir");
        let result = read_files();
        assert_eq!(result.len(), 2);
        result.iter().for_each(|t| match &t.name[..] {
            "menu" => {
                assert_eq!(3, t.fields.len());
                assert!(t.fields.iter().any(|item| {
                    *item
                        == UserTypeField {
                            name: "dish_name".to_string(),
                            type_name: "string".to_string(),
                            nullable: false,
                            layout_pos: 0,
                        }
                }));
                assert!(t.fields.iter().any(|item| {
                    *item
                        == UserTypeField {
                            name: "price".to_string(),
                            type_name: "int".to_string(),
                            nullable: false,
                            layout_pos: 1,
                        }
                }));
                assert!(t.fields.iter().any(|item| {
                    *item
                        == UserTypeField {
                            name: "description".to_string(),
                            type_name: "string".to_string(),
                            nullable: true,
                            layout_pos: 2,
                        }
                }));
            }
            "table" => {
                assert_eq!(2, t.fields.len());
                assert!(t.fields.iter().any(|item| {
                    *item
                        == UserTypeField {
                            name: "number".to_string(),
                            type_name: "int".to_string(),
                            nullable: false,
                            layout_pos: 0,
                        }
                }));
                assert!(t.fields.iter().any(|item| {
                    *item
                        == UserTypeField {
                            name: "occupied".to_string(),
                            type_name: "bit".to_string(),
                            nullable: false,
                            layout_pos: 1,
                        }
                }));
            }
            &_ => panic!(),
        });
    }
}
