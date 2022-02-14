pub struct BitVec {
    pub vec: Vec<u8>,
    length: usize,
}

impl BitVec {
    pub fn new() -> Self {
        BitVec {
            vec: Vec::new(),
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn read(&mut self, index: usize) -> bool {
        let cell = self.vec[index / 8];
        ((cell >> index % 8) & 1) == 1
    }

    pub fn write(&mut self, index: usize, value: bool) {
        while self.vec.len() < index / 8 + 1 {
            self.vec.push(0);
        }
        self.vec[index / 8] |= (value as u8) << (index % 8);
    }

    pub fn push(&mut self, value: bool) {
        self.write(self.len(), value);
        self.length += 1;
    }

    pub fn pop(&mut self) -> bool {
        let value = self.read(self.len() - 1);
        self.write(self.len() - 1, false);
        if self.len() / 8 < self.vec.len() {
            self.vec.pop();
        }
        value
    }
}
