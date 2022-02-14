use lildata::;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        let mut vector = BitVec::new();
        vector.write(5, true);
        assert_eq!(vector.vec[0], 0b00100000_u8);
    }

    #[test]
    fn test_read() {
        let mut vector = BitVec::new();
        let value = vector.read(5);
        assert_eq!(value, false);
    }
}