# cutr

> **cut** *cuts* select pieces of text from a file depending on `-b` byte position, `-c` character position, 
> `-d` delimiter, `-f` field.





Cut the first 20 characters to retrieve the author, then cut the date
```shell
cut -c 1-20 books.txt
# Author              
# Émile Zola          
# Samuel Beckett      
# Jules Verne

cut -c 21-25 books.txt   
```

Select the second and third column using the default `tab` delimiter
```shell
cut -f 2,3 books.tsv
```