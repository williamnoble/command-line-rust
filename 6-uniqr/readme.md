# Uniqr

> **Uniq** finds repeated adjacent lines in a file, it supports the argument `-c` which displays a frequency (count) of how many times
> a line has occurred. It supports input via stdin or supplying a file list as an argument. 

Example
```shell
a
a
b
a

# a
# b 
# a

# or with -c flag
# 2 a
# 1 b
# 1 a
```

Run with the `-c` flag to count
```shell
$ cargo run -- -c tests/inputs/three.txt
```
output
```
# 2 a
# 2 b
# 1 a
# 3 c
# 1 a
# 4 d
```