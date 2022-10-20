use core::mem::size_of;
use core::fmt::Debug;
use crate::CycleDelay;
use embedded_hal::digital::v2::{InputPin, OutputPin};


pub struct ShiftRegister<CLK, DATA, LATCH, VAL, DELAY>
where
    CLK: OutputPin,
    DATA: InputPin,
    LATCH: OutputPin,
{
    clk: CLK,
    data: DATA,
    latch: LATCH,
    phantom_data: core::marker::PhantomData<VAL>,
    phantom_delay: core::marker::PhantomData<DELAY>
}



pub trait ReadRegister<VAL>
{
    fn read(&mut self) -> VAL;
}

impl<CLK, DATA, LATCH, VAL, DELAY> ShiftRegister<CLK, DATA, LATCH, VAL,DELAY>
where
    CLK: OutputPin,
    DATA: InputPin,
    LATCH: OutputPin,
    DELAY: CycleDelay,
    VAL: core::ops::ShlAssign + Default + core::ops::AddAssign + From<bool> + Copy,
{
    pub fn new(clk: CLK, data: DATA, latch: LATCH) -> Self {
        Self {
            clk,
            data,
            latch,
            phantom_data: core::marker::PhantomData::default(),
            phantom_delay: core::marker::PhantomData::default(),
        }
    }
}

impl<CLK, DATA, LATCH, VAL, DELAY> ReadRegister<VAL> for ShiftRegister<CLK, DATA, LATCH, VAL, DELAY>
where
    CLK: OutputPin,
    DATA: InputPin,
    LATCH: OutputPin,
    DELAY: CycleDelay,
    VAL: core::ops::ShlAssign + Default + core::ops::AddAssign + From<bool> + Copy,
{
    fn read(&mut self) -> VAL {
        let mut v = VAL::default();
        let one: VAL = true.into();

        let bits = size_of::<VAL>() * 8;

        self.clk.set_low();
        self.latch.set_low();
        DELAY::delay();
        self.latch.set_high();

        for _ in 0..bits {
            v <<= one;
            DELAY::delay();

            if let Ok(true) = self.data.is_high() {
                v += one;
            }
            self.clk.set_high();
            DELAY::delay();

            self.clk.set_low();
        }

        v
    }
}