mod lcdc;
mod stat;
mod scy;
mod scx;
mod ly;
mod bgp;

use lcdc::RegLcdc;
use stat::RegStat;
use scy::RegScy;
use scx::RegScx;
use ly::RegLy;
use bgp::RegBgp;

use super::video_mode;

pub struct RegisterFile {
    pub lcdc: RegLcdc,
    pub stat: RegStat,
    pub scy: RegScy,
    pub scx: RegScx,
    pub ly: RegLy,
    pub bgp: RegBgp,
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            lcdc: RegLcdc::new(),
            stat: RegStat::new(),
            scy: RegScy::new(),
            scx: RegScx::new(),
            ly: RegLy::new(),
            bgp: RegBgp::new(),
        }
    }

    pub fn tick(&mut self) {
        if self.lcdc.lcd_en() {
            self.ly.tick();
        } else {
            self.ly.reset();
        }
    }
}
