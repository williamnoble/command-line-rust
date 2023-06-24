# Headr

> **Head** displays the first lines of a file, it accepts an argument for the number of lines or the number of bytes, and 
> a  list of zero or more files. If no file is specified input is taken from stdin. Headr is a simplified version.
---
## Examples

Run with the argument `lines` (returns x lines)
```shell
$ cargo run -- --lines 2  ./tests/inputs/ten.txt
# returns one\ntwo
```
Run with the argument `bytes` (returns x bytes)
```shell
$ cargo run -- --bytes 2  ./tests/inputs/ten.txt
# returns on
```

Run without a file specified, this reads input from stdin (supply via pipe)
```shell
$ echo "Hello" | cargo run -- --bytes 2
# returns He
```

## Notes on Code

We create type MyResult (similar to how result works in std::io), we return `Box<dyn Error>` to ensure that the trait 
is given a fixed size via fat pointer.
```type MyResult<T> = Result<T, Box<dyn Error>>;```

`.value_of` returns an Option<&str>

`.values_of_lossy` returns a Option<Vec<String>>

`.transpose()` returns a Result of an Option

` if let Some(num_bytes) = config.bytes` shorthand to match the value of the Option (Some or None).


Take x number of bytes from a file:
```rust
    .take(num_bytes as u64)
    .read(&mut buffer)?;
```