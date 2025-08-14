

use num::{Signed, cast::AsPrimitive};
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Subleq<T, M>
where
    T: Signed + AsPrimitive<usize>,
    M: Memory<T>,
{
    pub mem: M,
    pub curr_instruction: usize,
    #[doc(hidden)]
    _marker: std::marker::PhantomData<T>,
}

impl<T, M> Default for Subleq<T, M>
where
    T: Signed + AsPrimitive<usize>,
    M: Memory<T> + Default,
{
    fn default() -> Self {
        Self {
            mem: M::default(),
            curr_instruction: 0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Signed + AsPrimitive<usize>, M: Memory<T>> Subleq<T, M> {
    pub fn new(memory: M) -> Self {
        Self {
            mem: memory,
            curr_instruction: 0,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn step(&mut self) -> Result<(), M::Error> {
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

pub trait Memory<T>
where
    T: Signed + AsPrimitive<usize>,
{
    type Error: std::error::Error;

    fn get(&self, index: usize) -> Result<T, Self::Error>;

    fn set(&mut self, index: usize, value: T) -> Result<(), Self::Error>;
}
