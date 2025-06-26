use num::PrimInt;

pub trait Memory<T: PrimInt>: Default {
    fn get(&self, index: usize) -> T;
    fn set(&mut self, index: usize, value: T);
}

pub struct LinearMemory<T: PrimInt, const SIZE: usize>([T; SIZE]);

impl<T: PrimInt, const SIZE: usize> Default for LinearMemory<T, SIZE> {
    fn default() -> Self {
        Self([T::zero(); SIZE])
    }
}

impl<T: PrimInt, const SIZE: usize> Memory<T> for LinearMemory<T, SIZE> {
    fn get(&self, index: usize) -> T {
        self.0[index % SIZE]
    }

    fn set(&mut self, index: usize, value: T) {
        self.0[index % SIZE] = value;
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

    pub fn step(&mut self) {
        let (a_addr, b_addr, c_addr) = (
            self.mem
                .get(self.curr_instruction)
                .to_usize()
                .expect("cant convert number into usize address"),
            self.mem
                .get(self.curr_instruction.wrapping_add(1))
                .to_usize()
                .expect("cant convert number into usize address"),
            self.mem
                .get(self.curr_instruction.wrapping_add(2))
                .to_usize()
                .expect("cant convert number into usize address"),
        );

        let (a, b) = (self.mem.get(a_addr), self.mem.get(b_addr));

        let result = b - a;
        self.mem.set(b_addr, result);

        if result <= T::zero() {
            self.curr_instruction = c_addr;
        } else {
            self.curr_instruction = self.curr_instruction.wrapping_add(3)
        }
    }
}
