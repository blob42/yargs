# Colmap

CLI program that takes tabular data as input and maps a random
command to each input column. It works in a similar way to `awk` or `xargs` in
that the input text is converted into fields and the user supplied
commands are executed on each matched column (field)


## Example

INPUT:
-----

             field #1                 field #2
    |--------------------------|   |--------------|
    /long/path/to/some/ebook.pdf   | Title Of Ebook
                                     \
                                      \
    example usage:                     \
    --------------                      \____________________
                                                             \
    colmap --field-1='basename {}'  --field-2="awk { print $1 }"
    colmap -f1 'basename {}' -f2 'awk { print $1 }'

- use colon as delimiter 

    colmap -d':'

WILL OUPUT:
----------

ebook.pdf  |   Title











---
[I am using Github under protest](protest.md)


TODO:
----

[ ] use non-dashed approach to cli (rwxrob/bonzai)
