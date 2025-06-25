use num::PrimInt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Subleq<T: PrimInt, const MEM_SIZE: usize> {
    pub mem: [T; MEM_SIZE],
    pub curr_instruction: usize,
}

impl<T: PrimInt, const MEM_SIZE: usize> Default for Subleq<T, MEM_SIZE> {
    fn default() -> Self {
        Self {
            mem: [T::zero(); MEM_SIZE],
            curr_instruction: 0,
        }
    }
}

impl<T: PrimInt, const MEM_SIZE: usize> Subleq<T, MEM_SIZE> {
    const _ASSERT: () = assert!(MEM_SIZE != 0);

    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_memory(mem: [T; MEM_SIZE]) -> Self {
        Self {
            mem,
            ..Default::default()
        }
    }

    pub fn get(&self, index: usize) -> T {
        self.mem[index % MEM_SIZE]
    }

    pub fn set(&mut self, index: usize, value: T) {
        self.mem[index % MEM_SIZE] = value;
    }

    pub fn step(&mut self) {
        let (a_addr, b_addr, c_addr) = (
            self.get(self.curr_instruction)
                .to_usize()
                .expect("cant convert number into usize address"),
            self.get(self.curr_instruction.wrapping_add(1))
                .to_usize()
                .expect("cant convert numbner into usize address"),
            self.get(self.curr_instruction.wrapping_add(2))
                .to_usize()
                .expect("cant convert number into usize address"),
        );

        let (a, b, c) = (self.get(a_addr), self.get(b_addr), self.get(c_addr));

        let result = b - a;
        self.set(b_addr, result);

        if result <= T::zero() {
            self.curr_instruction = c
                .to_usize()
                .expect("cant convert number into usize address");
        } else {
            self.curr_instruction = self.curr_instruction.wrapping_add(3) % MEM_SIZE
        }
    }
}
