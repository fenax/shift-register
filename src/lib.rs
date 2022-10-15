#![no_std]

use core::mem::size_of;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::v2::{InputPin, OutputPin};

pub struct ShiftRegister<CLK, DATA, LATCH, VAL>
where
    CLK: OutputPin,
    DATA: InputPin,
    LATCH: OutputPin,
{
    clk: CLK,
    data: DATA,
    latch: LATCH,
    phantom_data: core::marker::PhantomData<VAL>,
}

pub trait ReadRegister<VAL> {
    fn read(&mut self) -> VAL;
}

impl<CLK, DATA, LATCH, VAL> ShiftRegister<CLK, DATA, LATCH, VAL>
where
    CLK: OutputPin,
    DATA: InputPin,
    LATCH: OutputPin,
    DATA::Error: core::fmt::Debug,
    VAL: core::ops::ShlAssign + Default + core::ops::AddAssign + From<bool> + Copy,
{
    pub fn new(clk: CLK, data: DATA, latch: LATCH) -> Self {
        Self {
            clk,
            data,
            latch,
            phantom_data: core::marker::PhantomData::default(),
        }
    }
}
impl<CLK, DATA, LATCH, VAL> ReadRegister<VAL> for ShiftRegister<CLK, DATA, LATCH, VAL>
where
    CLK: OutputPin,
    DATA: InputPin,
    LATCH: OutputPin,
    DATA::Error: core::fmt::Debug,
    VAL: core::ops::ShlAssign + Default + core::ops::AddAssign + From<bool> + Copy,
{
    fn read(&mut self) -> VAL {
        let mut v = VAL::default();
        let one: VAL = true.into();

        let bits = size_of::<VAL>() * 8;

        _ = self.clk.set_low();
        _ = self.latch.set_low();
        cortex_m::asm::delay(10);
        _ = self.latch.set_high();

        for _ in 0..bits {
            v <<= one;
            cortex_m::asm::delay(10);

            if self.data.is_high().unwrap() {
                v += one;
            }
            _ = self.clk.set_high();
            cortex_m::asm::delay(10);

            _ = self.clk.set_low();
        }

        v
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
