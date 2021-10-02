![Build status](https://github.com/helmutschneider/brainfuck/workflows/build/badge.svg)

# Brainfuck interpreter in Rust
This repository contains a tiny implementation of the [Brainfuck language](https://en.wikipedia.org/wiki/Brainfuck).

A brainfuck program is written in clear text and only contains eight tokens of relevance. Anything else is silently ignored.

| Character | Implemented | Meaning                                                              |
|-----------|-------------|----------------------------------------------------------------------|
| >         | Yes         | Increment the data pointer (to point to the next cell to the right). |
| <         | Yes         | Decrement the data pointer (to point to the next cell to the left).  |
| +         | Yes         | Increment (increase by one) the byte at the data pointer.            |
| -         | Yes         | Decrement (decrease by one) the byte at the data pointer.            |
| .         | Yes         | Output the byte at the data pointer.                                 |
| ,         | No          | Accept one byte of input, storing its value in the byte at the data pointer. |
| [         | Yes         | If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command. |
| ]         | Yes         | If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command. |

## Run the tests
```
cargo test
```
