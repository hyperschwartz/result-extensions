//! # Result Extensions
//!
//! A simple library that functionally creates Result<T, E> values from arbitrary types.
//!
//! Usage:
//! ```
//! mod some_mod {
//! use result_extensions::ResultExtensions;
//!     fn is_greater_than_ten(input: i64) -> Result<bool, String> {
//!         match input {
//!             i64::MIN..=0 => {
//!                 "this function does not accept values less than or equal to zero for some reason"
//!                     .to_string()
//!                     .to_err()
//!             }
//!             1..=9 => false.to_ok(),
//!             _ => true.to_ok(),
//!         }
//!     }
//! }

/// Allows any Sized type to be functionally moved into a Result<T, E>.
pub trait ResultExtensions
where
    Self: Sized,
{
    /// Converts the caller into an Ok (left-hand-side) Result.
    fn to_ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }

    /// Converts the caller into an Err (right-hand-side) Result.
    fn to_err<T>(self) -> Result<T, Self> {
        Err(self)
    }
}
impl<T> ResultExtensions for T {}

#[cfg(test)]
mod tests {
    use crate::ResultExtensions;

    #[test]
    fn test_to_ok() {
        let result = get_ok("ok");
        if let Ok(ok_text) = result {
            assert_eq!("ok", ok_text, "unexpected Ok text",);
        } else {
            panic!("expected result to be an Ok variant, but got: {:?}", result);
        }
    }

    #[test]
    fn test_to_err() {
        let result = get_err("error");
        if let Err(err_text) = result {
            assert_eq!("error", err_text, "unexpected Err text",);
        } else {
            panic!(
                "expected result to be an Err variant, but got: {:?}",
                result
            );
        }
    }

    fn get_ok<S: Into<String>>(ok_value: S) -> Result<String, String> {
        ok_value.into().to_ok()
    }

    fn get_err<S: Into<String>>(err_value: S) -> Result<String, String> {
        err_value.into().to_err()
    }
}
