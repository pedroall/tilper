pub mod db;

#[test]
fn db_get() {
    let mut database = db::Database::new();
    let data = database
        .set(
            "boo".to_string(),
            "far".to_string()
        ).unwrap()
        .get(
            "boo".to_string()
        ).unwrap();

    assert_eq!("far", data);
}
