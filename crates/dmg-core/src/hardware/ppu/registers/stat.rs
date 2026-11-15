pub struct RegStat(u8);

impl RegStat {
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
