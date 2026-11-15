use super::video_mode::VideoMode;

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

    pub fn reset(&mut self) {
        self.value = 0;
        self.line_cycles = 0;
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

    // Mode checks
    pub fn get_mode(&self) -> VideoMode {
        if self.value >= 144 {
            VideoMode::Mode1
        }
        else if self.line_cycles < 20 {
            VideoMode::Mode2
        }
        else if self.line_cycles < 63 {
            VideoMode::Mode3
        }
        else {
            VideoMode::Mode0
        }
    }
    
    pub fn at_mode_start(&self) -> bool {
        let mode = self.get_mode();
        match mode {
            VideoMode::Mode0 => (self.line_cycles == 63) && (self.value < 144),
            VideoMode::Mode1 => (self.line_cycles == 0) && (self.value == 144),
            VideoMode::Mode2 => (self.line_cycles == 0) && (self.value < 144),
            VideoMode::Mode3 => (self.line_cycles == 20) && (self.value < 144),
        }
    }
    
    pub fn at_vblank_start(&self) -> bool {
        (self.value == 144) && (self.line_cycles == 0)
    }
}

