pub mod tilper;

#[test]
fn db_get() {
    let mut db = tilper::Tilper::new();
    let data = db
        .set(
            "boo".to_string(),
            "far".to_string()
        ).unwrap()
        .get(
            "boo".to_string()
        ).unwrap();

    assert_eq!("far", data);
}
