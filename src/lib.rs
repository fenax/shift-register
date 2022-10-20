#![no_std]

pub mod input;

pub trait CycleDelay{
    fn delay();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
