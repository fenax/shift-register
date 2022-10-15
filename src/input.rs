use core::mem::size_of;
use core::fmt::Debug;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::v2::{InputPin, OutputPin};

#[derive(Debug)]
pub struct Error<HWERROR>
where 
HWERROR:Debug{
    parent:HWERROR  
}

impl<HWERROR> From<HWERROR> for Error<HWERROR>
where HWERROR:Debug
{
    fn from(error:HWERROR)->Self{
        Self{
            parent:error
        }
    }
}

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

pub trait ReadRegister<VAL,ERR>
where
ERR:Debug
{
    fn read(&mut self) -> Result< VAL,Error<ERR>>;
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
impl<CLK, DATA, LATCH, VAL,ERR> ReadRegister<VAL,ERR> for ShiftRegister<CLK, DATA, LATCH, VAL>
where
    CLK: OutputPin,
    DATA: InputPin,
    LATCH: OutputPin,
    ERR: Debug,
    DATA::Error: core::fmt::Debug,
    VAL: core::ops::ShlAssign + Default + core::ops::AddAssign + From<bool> + Copy,
{
    fn read(&mut self) -> Result<VAL,Error<ERR>> {
        let mut v = VAL::default();
        let one: VAL = true.into();

        let bits = size_of::<VAL>() * 8;

        self.clk.set_low()?;
        self.latch.set_low()?;
        //cortex_m::asm::delay(10);
        self.latch.set_high()?;

        for _ in 0..bits {
            v <<= one;
            //cortex_m::asm::delay(10);

            if self.data.is_high().unwrap() {
                v += one;
            }
            self.clk.set_high()?;
            //cortex_m::asm::delay(10);

            self.clk.set_low()?;
        }

        Ok(v)
    }
}