
use num::{
    Signed,
    traits::{WrappingAdd, WrappingSub},
};

pub struct Subleq<T, M>
where
    T: Signed + WrappingAdd + WrappingSub + From<i8> + Copy,
    M: Memory<T>,
{
    pub mem: M,
    pub curr_instruction: T,
    #[doc(hidden)]
    _marker: std::marker::PhantomData<T>,
}

impl<T, M> Default for Subleq<T, M>
where
    T: Signed + WrappingAdd + WrappingSub + From<i8> + Copy,
    M: Memory<T> + Default,
{
    fn default() -> Self {
        Self::new(M::default())
    }
}

impl<T, M> Subleq<T, M>
where
    T: Signed + WrappingAdd + WrappingSub + From<i8> + Copy,
    M: Memory<T>,
{
    pub fn new(memory: M) -> Self {
        Self {
            mem: memory,
            curr_instruction: T::zero(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn step(&mut self) -> Result<(), M::Error> {
        let (a, b, c) = self.mem.instruction(&self.curr_instruction)?;

        let (a_value, b_value) = (self.mem.get(a)?, self.mem.get(&b)?);

        let result = b_value.wrapping_sub(a_value);

        if !b_value.is_positive() {
            self.curr_instruction = *c;
        } else {
            self.curr_instruction = self.curr_instruction.wrapping_add(&T::from(3i8));
        }

        self.mem.set_value(&b, result)?;
        Ok(())
    }
}

pub trait Memory<T>
where
    T: WrappingAdd + From<i8> + Copy,
{
    /// An error while using the memory
    type Error: std::error::Error;

    fn get(&self, index: &T) -> Result<&T, Self::Error>;

    fn instruction(&self, index: &T) -> Result<(&T, T, &T), Self::Error> {
        Ok((
            self.get(index)?,
            *self.get(&index.wrapping_add(&T::from(1i8)))?,
            self.get(&index.wrapping_add(&T::from(2i8)))?,
        ))
    }

    fn set(&mut self, index: &T, value: T) -> Result<(), Self::Error>;
}
