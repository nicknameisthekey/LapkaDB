use std::{env, fs, fmt::format};

use crate::usertypes;

pub fn new(coll_type: String) {
    let app_dir = env::var("LAPKA_FILES").unwrap();
    let t = usertypes::by_name(coll_type);
    fs::write(format!("{}/collections/{}", app_dir, t.name), "").unwrap();
}

#[test]
fn creating_new_collection() {
    env::set_var("LAPKA_FILES", "test_dir3");
    new("menu".to_string());
    let dir_data = fs::read_dir("test_dir3/collections").unwrap();
}
