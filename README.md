# bf-rust-interpreter

An interpreter for the Brainfuck programming language written entirely in Rust. 

This was written without looking at the source of any other Brainfuck interpreters.

## How to run
```shell
cargo run hello_world.bf
```

## Brainfuck 

### Standard Brainfuck characters
These are the characters that are a part of the standard Brainfuck specification. 
```text
< - Move pointer left
> - Move pointer right
+ - Increment value at the current cell (this operation CAN overflow)
- - Decrement value at the current cell (this operation CAN underflow)
[ - Begin loop (loop will be broken when value at current point is 0)
] - Jump to beginning of loop 
. - Outputs the current value of a cell (ASCII)
, - Read input and put ASCII value in current cell
```

### Non-Standard Brainfuck characters
These are characters added by me to give more flexibility to developers.
```text
* - Inserts a random value in a cell
# - Outputs the current value of a cell (Integer value)
```
