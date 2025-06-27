mod error;
pub mod memory;
pub use error::Error;
pub use memory::Memory;

use num::{Signed, Zero, cast::AsPrimitive};

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
