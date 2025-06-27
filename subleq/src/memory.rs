use crate::Error;
use num::{Signed, Zero, cast::AsPrimitive};

pub trait Memory<T: Signed + Zero + AsPrimitive<usize>>: Default {
    fn get(&self, index: usize) -> Result<T, Error>;
    fn set(&mut self, index: usize, value: T) -> Result<(), Error>;
}

pub struct LinearMemory<T: Signed + Zero + AsPrimitive<usize>, const SIZE: usize>([T; SIZE]);

impl<T: Signed + Zero + AsPrimitive<usize>, const SIZE: usize> Default for LinearMemory<T, SIZE> {
    fn default() -> Self {
        Self([T::zero(); SIZE])
    }
}

impl<T: Signed + Zero + AsPrimitive<usize>, const SIZE: usize> Memory<T> for LinearMemory<T, SIZE> {
    fn get(&self, address: usize) -> Result<T, Error> {
        self.0
            .get(address)
            .ok_or(Error::AddressOutOfRange(address))
            .copied()
    }

    fn set(&mut self, address: usize, value: T) -> Result<(), Error> {
        let reference = self
            .0
            .get_mut(address)
            .ok_or(Error::AddressOutOfRange(address))?;

        *reference = value;
        Ok(())
    }
}
