pub struct RegBgp(u8);

impl RegBgp {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn read(&self) -> u8 {
        self.0
    }

    pub fn write(&mut self, value: u8) {
        self.0 = value
    }
}
