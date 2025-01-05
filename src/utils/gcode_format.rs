use crate::prelude::*;

use defmt::Format;
pub use gcode::GCode;

impl Format for W<&GCode> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "GCode {{ Major-number: {}, mnemonic-str: {} }}",
            self.0.major_number(),
            //Fixme: this line is expensive and use core::fmt on device
            defmt::Debug2Format(&self.0.mnemonic())
        );
    }
}
