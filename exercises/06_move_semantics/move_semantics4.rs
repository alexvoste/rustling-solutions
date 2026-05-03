fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        x.push(42);
        let y = x.clone();
        let mut z = y.clone();
        z.push(13);
        assert_eq!(y, [42]);
        assert_eq!(z, [42, 13]);
    }
}
