fn main() {
}

#[allow(dead_code)]
struct BitVec {
    vec: Vec<u8>,
    length: usize
}

#[allow(dead_code)]
impl BitVec {
    pub fn new() -> Self {
        BitVec {
            vec: vec![0],
            length: 0
        }
    }

    pub fn read(&mut self, index: usize) -> bool {
        let cell = self.vec[index/8];
        ((cell >> index % 8) & 1) == 1
    }

    pub fn write(&mut self, index: usize, value: bool) {
        self.vec[index / 8] |= (value as u8) << (index % 8);
    }
}

mod funcs {
    
}