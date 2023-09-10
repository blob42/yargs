Very much work in progress. This is the first project in my Rust learning journey. 
Expect a lot of changes and refactoring.

# Colmap

Colmap is hybrid between `awk` in `xargs`. It borrows from the former its ability
to work on tabular columns of text and allows for arbitrary commands to be
applied per column in a similar way to `xargs`.

The columns are called `fields`. The command to execute on each field is called
an `x-arg`. 


## Example

input:

             field #1                 field #2
    |--------------------------|   |--------------|
    /long/path/to/some/ebook.pdf   | Title Of Ebook
                                     ____
                                       |
    example usage:                     | x:arg
    --------------                     |                     
                          ----------------                       
    colmap 'basename {}' "awk { print $1 }"
                                                    

    #OR
    colmap -f1 'basename {}' -f2 'awk { print $1 }'

would output: `ebook.pdf  |   Title`


- use colon as delimiter 

`colmap -d':' -f1 '...'`

### Ways of passing x-args

1. passing column x-args as fields

```shell
foo_cmd | colmap --field-1='basename {}'  --field-2="awk { print $1 }"
foo_cmd | colmap -f1 'basename {}' -f2 'awk { print $1 }'
```

2. Passing `xargs` as positional arguments

```shell
foo_cmd | colmap 'basename {}' 'awk { print $2 }'
```


---
[I am using Github under protest](protest.md)


