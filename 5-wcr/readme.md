# wcr

> **wc** counts `-l` lines, `-w` words, `-c` bytes and `-c` characters, it can read from stdin via pipe or 
> from a list of files. If no argument is specified it shows lines, words, and bytes (no chars).

Default run (input via stdin pipe) without supplying any arguments, defaults to `lines - words - bytes`
```shell
$ echo "Moving Slowly With Grace" | cargo run
#  L       W      B     (illustrative)
#  1       4      25
```

Run with the `-lines --l` flag
```shell
$ echo "Moving Slowly With Grace" | cargo run -- -l
#  L       W      B     (illustrative)
#  1       4      25
```

Run by specifying a file
```shell
$ cargo run -- tests/inputs/atlamal.txt 
 4      29     177 tests/inputs/atlamal.txt
```

Run by specifying a file
```shell
$ cargo run -- tests/inputs/atlamal.txt 
 4      29     177 tests/inputs/atlamal.txt
```

Confirm that the chars option works
```shell
echo "2014" | wc -c
printf "2014" | wc -c
```

```shell
$ cargo run -- one.txt
1       1       3 one.txt

```
