mod usertypes {
    struct UserType {
        name: String,
        fields: Vec<UserTypeField>,
    }
    struct UserTypeField {
        name: String,
        type_name: String,
        nullable: bool,
        layout_pos: u8,
    }

    impl UserType {
        fn from_file_string(file_string: String) -> UserType {
            let mut lines = file_string.split("\r\n");
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

    #[test]
    fn it_works() {
        let result = UserType::from_file_string("type_name\r\nname,string,0,0".to_string());
        assert_eq!("type_name", result.name);
        assert_eq!(1, result.fields.len());
        assert_eq!("name", result.fields[0].name);
        assert_eq!("string", result.fields[0].type_name);
        assert_eq!(false, result.fields[0].nullable);
        assert_eq!(0, result.fields[0].layout_pos);
    }
}
