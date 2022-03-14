# Commands

## Install

```sh
cargo install tilper
```

## Test

```sh
cargo test
```

# Example

### Creating a database and passing some values
```rs
use tilper::Tilper;

fn main() {
  let mut db = Tilper::new();
  let data = db
    .set("foo".to_string(), "bar".to_string()).unwrap()
    .get("foo").unwrap();
  assert_eq!("far", data); // Panic
}
```
