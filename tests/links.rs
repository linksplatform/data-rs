use platform_data::Error;
use std::borrow::Cow;

#[test]
fn test_error_not_exists_display() {
    let err: Error<'_, u64> = Error::NotExists(42);
    assert_eq!(format!("{err}"), "link 42 does not exist.");
}

#[test]
fn test_error_has_usages_display() {
    let usages = vec![Cow::Owned(vec![1u64, 2, 3])];
    let err: Error<'_, u64> = Error::HasUsages(usages);
    assert!(format!("{err}").contains("has dependencies"));
}

#[test]
fn test_error_already_exists_display() {
    let err: Error<'_, u64> = Error::AlreadyExists(Cow::Owned(5));
    assert_eq!(format!("{err}"), "link 5 already exists");
}

#[test]
fn test_error_limit_reached_display() {
    let err: Error<'_, u64> = Error::LimitReached(1000);
    assert!(format!("{err}").contains("limit"));
    assert!(format!("{err}").contains("1000"));
}

#[test]
fn test_error_alloc_failed_from_io_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::OutOfMemory, "no memory");
    let err: Error<'_, u64> = Error::from(io_err);
    assert!(format!("{err}").contains("unable to allocate"));
}

#[test]
fn test_error_other_from_box() {
    let boxed: Box<dyn std::error::Error + Sync + Send> = "custom error".into();
    let err: Error<'_, u64> = Error::from(boxed);
    assert!(format!("{err}").contains("custom error"));
}

#[test]
fn test_error_debug() {
    let err: Error<'_, u64> = Error::NotExists(7);
    let debug_str = format!("{err:?}");
    assert!(debug_str.contains("NotExists"));
}
