# Result Extensions

An extremely simple library that provides extension traits for the standard Rust library's `Result<T, E>` type.

This library adds "extension functions" to all `Sized` values to allow them to be moved into `Result` types:
```rust
use result_extensions::ResultExtensions;

fn result_function(bool: is_err) -> Result<String, String> {
  if is_err {
    "error!".to_string().to_err()
  } else {
    "ok!".to_string().to_ok()
  }
}
```

