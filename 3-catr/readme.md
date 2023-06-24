# Catr

> **cat** will print to stdout the contents of one or more files, it accepts arguments -n print line numbers, -b 
> which prints line numbers for non-blank lines only, it is not permissible to allow flags -n and -b together.

Run without any arguments, prints the file output without line numbers
```shell
$ cargo run -- ./tests/inputs/the-bustle.txt
# The bustle in a house
# The morning after death
# Is solemnest of industries
# Enacted upon earth,—
# 
# The sweeping up the heart,
# And putting love away
# We shall not want to use again
# Until eternity.
```

Prints the file output with line numbers
```shell
$ cargo run -- ./tests/inputs/the-bustle.txt -n
# 1  The bustle in a house
# 2  The morning after death
# 3  Is solemnest of industries
# 4  Enacted upon earth,—
# 5  
# 6  The sweeping up the heart,
# 7  And putting love away
# 8  We shall not want to use again
# 9  Until eternity.
```

Prints the file output with line numbers ignoring the numbering of any blank lines
```shell
$ cargo run -- ./tests/inputs/the-bustle.txt -b
# 1  The bustle in a house
#2  The morning after death
#3  Is solemnest of industries
#4  Enacted upon earth,—
#
#5  The sweeping up the heart,
#6  And putting love away
#7  We shall not want to use again
#8  Until eternity.
```