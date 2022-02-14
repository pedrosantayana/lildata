use lildata::BitVec;

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
    fn test_push() {
        let mut vector = BitVec::new();
        vector.push(true);
        assert_eq!(vector.vec[0], 1);
    }

    #[test]
    fn test_push_many_and_overflow() {
        let mut vector = BitVec::new();
        for _ in 0..9 {
            vector.push(true);
        }
        assert_eq!(vector.len(), 9);
        assert_eq!(vector.vec[0], 0b11111111);
        assert_eq!(vector.vec[1], 0b00000001);
    }

    #[test]
    fn test_pop() {
        let mut vector = BitVec::new();
        vector.write(5, true);
        vec
    }
}