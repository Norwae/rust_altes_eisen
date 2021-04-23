mod keywords;
mod ownership;
mod generic;
mod modular {
    // see the cargo.toml file for this
}
mod safety;
mod futures;

fn main() {
    generic::sizing()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple_arithmetic() {
        assert_eq!(4, 2 + 2)
    }
}