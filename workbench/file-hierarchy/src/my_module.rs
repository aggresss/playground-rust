// src/my_module.rs (any file of your project)

fn f() -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::f; // Need to import items from parent module. Has
                  // access to non-public members.
    #[test]
    fn ff() {
        assert_eq!(f(), 0);
    }
}
