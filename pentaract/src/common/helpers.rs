/// Like `Result::ok`, but otherwise
pub fn not_ok<T, E>(res: Result<T, E>) -> Option<E> {
    match res {
        Err(e) => Some(e),
        _ => None,
    }
}
