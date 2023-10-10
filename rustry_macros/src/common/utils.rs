pub fn opt_false(b: &bool) -> bool {
    !(*b)
}

pub fn opt_none<T>(v: &Option<T>) -> bool {
    v.is_none()
}
