use num::{Signed, Zero, cast::AsPrimitive};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("address `{0}` is out of range for memory")]
    AddressOutOfRange(usize),
    #[error("immutable memory address `{0}`")]
    ImmutableAddress(usize),
    #[error("custom error: {0}")]
    Custom(#[source] Box<dyn std::error::Error>),
}

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Subleq<T: Signed + Zero + AsPrimitive<usize>, M: Memory<T>> {
    pub mem: M,
    pub curr_instruction: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Signed + Zero + AsPrimitive<usize>, M: Memory<T>> Default for Subleq<T, M> {
    fn default() -> Self {
        Self {
            mem: M::default(),
            curr_instruction: 0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Signed + Zero + AsPrimitive<usize>, M: Memory<T>> Subleq<T, M> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_memory(mem: M) -> Self {
        Self {
            mem,
            curr_instruction: 0,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn step(&mut self) -> Result<(), Error> {
        let (a, b, c): (usize, usize, usize) = (
            self.mem.get(self.curr_instruction)?.as_(),
            self.mem.get(self.curr_instruction.wrapping_add(1))?.as_(),
            self.mem.get(self.curr_instruction.wrapping_add(2))?.as_(),
        );

        let (a_value, b_value) = (self.mem.get(a)?, self.mem.get(b)?);

        let result = b_value - a_value;
        self.mem.set(b, result)?;

        if !result.is_positive() {
            self.curr_instruction = c;
        } else {
            self.curr_instruction = self.curr_instruction.wrapping_add(3)
        }
        Ok(())
    }
}
