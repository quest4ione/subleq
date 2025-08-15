Interpret a subleq program.

Subleq is an instruction set which contains only one instruction: subleq.
A subleq computer has a single memory unit in which both the program and its data is stored.
The subleq instruction has three arguments: A, B and C.
Firstly, the computer substracts the value at address A from the value at address B.
Then it stores the result at address B. If the result is smaller than or equals 0,
the computer jumps to address C. If it isn't, the computer continues to the next instruction.
```text
read instruction -> (A, B, C)
MEM[B] = MEM[B] - MEM[A]
if MEM[B] <= 0 {
    jump C
} else {
    jump curr_instruction + 3
}
```

## Documentation
<https://docs.rs/qelsub/>

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
