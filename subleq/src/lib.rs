use num::PrimInt;

#[non_exhaustive]
pub enum Error {
    CantConvertValueToUsize,
    AddressOutOfRange(usize),
}

pub trait Memory<T: PrimInt>: Default {
    fn get(&self, index: usize) -> Result<T, Error>;
    fn set(&mut self, index: usize, value: T) -> Result<(), Error>;
}

pub struct LinearMemory<T: PrimInt, const SIZE: usize>([T; SIZE]);

impl<T: PrimInt, const SIZE: usize> Default for LinearMemory<T, SIZE> {
    fn default() -> Self {
        Self([T::zero(); SIZE])
    }
}

impl<T: PrimInt, const SIZE: usize> Memory<T> for LinearMemory<T, SIZE> {
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
pub struct Subleq<T: PrimInt, M: Memory<T>> {
    pub mem: M,
    pub curr_instruction: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T: PrimInt, M: Memory<T>> Default for Subleq<T, M> {
    fn default() -> Self {
        Self {
            mem: M::default(),
            curr_instruction: 0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: PrimInt, M: Memory<T>> Subleq<T, M> {
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
        let (a, b, c) = (
            self.mem
                .get(self.curr_instruction)?
                .to_usize()
                .ok_or(Error::CantConvertValueToUsize)?,
            self.mem
                .get(self.curr_instruction.wrapping_add(1))?
                .to_usize()
                .ok_or(Error::CantConvertValueToUsize)?,
            self.mem
                .get(self.curr_instruction.wrapping_add(2))?
                .to_usize()
                .ok_or(Error::CantConvertValueToUsize)?,
        );

        let (a_value, b_value) = (self.mem.get(a)?, self.mem.get(b)?);

        let result = b_value - a_value;
        self.mem.set(b, result)?;

        if result <= T::zero() {
            self.curr_instruction = c;
        } else {
            self.curr_instruction = self.curr_instruction.wrapping_add(3)
        }
        Ok(())
    }
}
