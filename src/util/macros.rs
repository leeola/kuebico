/// Wrap the returned error value with Some(), mainly useful for Iterator returns.
macro_rules! try_some {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            return Some(Err(From::from(err)))
        }
    })
}

/// Get the Some(value), returning None otherwise.
macro_rules! something {
    ($expr:expr) => (match $expr {
        Some(val) => val,
        None => {
            return None
        }
    })
}
