pub struct RegLy {
    value: usize,
    line_cycles: usize,
}

impl RegLy {
    pub fn new() -> Self {
        Self {
            value: 0,
            line_cycles: 0,
        }
    }

    pub fn read(&self) -> u8 {
        self.value as u8
    }

    pub fn tick(&mut self) {
        self.line_cycles += 1;
        if self.line_cycles >= 114 {
            self.value += 1;
            self.line_cycles = 0;
        }
        if self.value > 153 {
            self.value = 0;
        }
    }

    pub fn at_line_start(&self) -> bool {
        self.line_cycles == 0
    }

    pub fn at_vblank_start(&self) -> bool {
        (self.value == 144) && (self.line_cycles == 0)
    }
}
