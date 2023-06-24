findr

> **find** is used to walk a filepath, you can specify `-t` type which could include `files, links, directories`, 
> the `-n` name option finds files matching a given file blob e.g. `*.csv`.






Sanity check for Walk Path
```shell
cargo run -- tests/inputs/a/
# tests/inputs/a/
# tests/inputs/a/a.txt
# tests/inputs/a/b
# tests/inputs/a/b/b.csv
# tests/inputs/a/b/c
# tests/inputs/a/b/c/c.mp3
```