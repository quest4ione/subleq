//! Customizable [subleq](https://esolangs.org/wiki/Subleq) instruction set program execution.
//!
//! Provides a subleq interpreter and a trait to customize memory mappings.
//!
//! See [Subleq] for an explanation of the instruction set.
//!
//! See [Subleq] and [Memory] for usage examples.
#![deny(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc
)]

use num::{
    Signed,
    traits::{WrappingAdd, WrappingSub},
};

/// Interpret a subleq program stored inside a [Memory].
///
/// Subleq is a instruction set which contains only one instruction: subleq.
/// A subleq computer has a single memory unit in which both the program and its data is stored.
/// The subleq instruction has three arguments: A, B and C.
/// Firstly, the computer substracts the value at address A from the value at address B.
/// Then it stores the result at address B. If the result is smaller than or equals 0,
/// the computer jumps to address C. If it isn't, the computer continues to the next instruction.
/// ```text
/// read instruction -> (A, B, C)
/// MEM[B] = MEM[B] - MEM[A]
/// if MEM[B] <= 0 {
///     jump C
/// } else {
///     jump curr_instruction + 3
/// }
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Subleq<T, M>
where
    T: Signed + WrappingAdd + WrappingSub + From<i8> + Copy,
    M: Memory<T>,
{
    /// The memory that the subleq program is stored in.
    pub mem: M,
    /// The address of the first argument of the instruction which is going to be executed next.
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
    /// Construct a new [Subleq] struct from a [Memory].
    /// If the [Memory] implementation implements default, Subleq::default can be used.
    ///
    /// Construct a new [Subleq] struct from a [Memory] that starts execution at the first address.
    ///
    /// ```
    /// # use subleq::{Subleq, Memory};
    /// # struct ByteMemory([i8; 256]);
    /// #
    /// # impl Memory<i8> for ByteMemory {
    /// #   type Error = std::convert::Infallible;
    /// #
    /// #   fn get(&self, index: &i8) -> Result<&i8, Self::Error> {
    /// #     Ok(&self.0[*index as u8 as usize])
    /// #   }
    /// #   fn set(&mut self, index: &i8, value: i8) -> Result<(), Self::Error> {
    /// #     self.0[*index as u8 as usize] = value;
    /// #     Ok(())
    /// #   }
    /// # }
    /// # impl ByteMemory {
    /// #   fn new() -> Self { Self([0; 256]) }
    /// # }
    /// let memory = ByteMemory::new();
    /// let subleq = Subleq::new(memory);
    /// ```
    pub fn new(memory: M) -> Self {
        Self {
            mem: memory,
            curr_instruction: T::zero(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Execute the current instruction.
    ///
    /// A subleq instruction has three arguments: A, B and C. Its execution consists of two steps:
    /// 1. SUB: substract the value at address A from the value at B and store it in B.
    /// 2. LEQ: if the above result is less than or equal to 0,
    ///    set the instruction pointer to address C. Otherwise set it to the next instruction.
    ///
    /// ```no_run
    /// # use subleq::{Subleq, Memory};
    /// # struct ByteMemory([i8; 256]);
    /// #
    /// # impl Memory<i8> for ByteMemory {
    /// #   type Error = std::convert::Infallible;
    /// #
    /// #   fn get(&self, index: &i8) -> Result<&i8, Self::Error> {
    /// #     Ok(&self.0[*index as u8 as usize])
    /// #   }
    /// #   fn set(&mut self, index: &i8, value: i8) -> Result<(), Self::Error> {
    /// #     self.0[*index as u8 as usize] = value;
    /// #     Ok(())
    /// #   }
    /// # }
    /// # impl ByteMemory {
    /// #   fn new() -> Self { Self([0; 256]) }
    /// # }
    /// let memory = ByteMemory::new();
    /// let mut subleq = Subleq::new(memory);
    /// while let Ok(_) = subleq.step() { }
    /// ```
    ///
    /// # Errors
    /// Returns an [Memory::Error] when getting or setting [Memory] fails.
    /// The error type is specific to the [Memory] implementation.
    pub fn step(&mut self) -> Result<(), M::Error> {
        let (a, b, c) = self.mem.instruction(&self.curr_instruction)?;

        let (a_value, b_value) = (self.mem.get(a)?, self.mem.get(&b)?);

        let result = b_value.wrapping_sub(a_value);

        if !b_value.is_positive() {
            self.curr_instruction = *c;
        } else {
            self.curr_instruction = self.curr_instruction.wrapping_add(&T::from(3i8));
        }

        self.mem.set(&b, result)?;
        Ok(())
    }
}

/// Represent a read- and writable Memory implementation.
///
/// Example implementation
/// ```
/// # use subleq::Memory;
/// struct ByteMemory([i8; 256]);
///
/// impl Memory<i8> for ByteMemory {
///   type Error = std::convert::Infallible;
///
///   fn get(&self, index: &i8) -> Result<&i8, Self::Error> {
///     Ok(&self.0[*index as u8 as usize])
///   }
///
///   fn set(&mut self, index: &i8, value: i8) -> Result<(), Self::Error> {
///     self.0[*index as u8 as usize] = value;
///     Ok(())
///   }
/// }
/// ```
pub trait Memory<T>
where
    T: WrappingAdd + From<i8> + Copy,
{
    /// An error while using the memory
    type Error: std::error::Error;

    /// Get the value at an address or return an error.
    ///
    /// # Errors
    /// Errors are implementation-specific, see [Self::Error].
    fn get(&self, index: &T) -> Result<&T, Self::Error>;

    /// Get the instruction at an address or return an error.
    ///
    /// The provided implementation calls [Self::get].
    ///
    /// # Errors
    /// Errors are implementation-specfific, see [Self::Error].
    fn instruction(&self, index: &T) -> Result<(&T, T, &T), Self::Error> {
        Ok((
            self.get(index)?,
            *self.get(&index.wrapping_add(&T::from(1i8)))?,
            self.get(&index.wrapping_add(&T::from(2i8)))?,
        ))
    }

    /// Set the value at an address or return an error.
    ///
    /// # Errors
    /// Errors are implementation-specific, see [Self::Error].
    fn set(&mut self, index: &T, value: T) -> Result<(), Self::Error>;
}
