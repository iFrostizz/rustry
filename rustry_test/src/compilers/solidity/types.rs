pub fn internal_to_type(_type: &str) -> String {
    if _type.ends_with(']') {
        unimplemented!("{_type} is an invalid type for now");
    }
    if _type.starts_with("tuple") {
        unimplemented!("{_type} variants are not yet unpacked");
    }
    if _type.starts_with("uint") {
        // TODO clamp types
        // String::from("U256")
        String::from("u128")
    } else {
        todo!("{_type} missing !");
    }
}
