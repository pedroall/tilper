pub mod db;

#[test]
fn db_get() {
    let mut database = db::Database::new("db".to_string());
    let data = database.get("foo".to_string()).unwrap();

    assert_eq!("bar", data);
    println!("{:?}", database);
}
