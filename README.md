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
```rust
use tilper::db::{Database};

fn main() {
  let mut db = Database::new();
  let data = db
    .set("foo".to_string(), "bar".to_string()).unwrap()
    .get("foo").unwrap();
  assert_eq!("far", data); // Panic
}
```
