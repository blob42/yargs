Very much work in progress. This is the first project in my Rust learning journey. 
Expect a lot of changes and refactoring.

# yargs

`yargs` aims to be the `xargs` equivalent to tabular input. It borrows from `awk`
the its ability to work on columns of text and allows for arbitrary
commands to be applied per column in a similar way to `xargs`.

The columns are called `fields`. The command to execute on each field is called
an `x-arg`. 

# Usage

1. passing column yargs as fields

```shell
foo_cmd | yargs --field-1='basename {}'  --field-2="awk { print $1 }"
foo_cmd | yargs -f1 'basename {}' -f2 'awk { print $1 }'
```

2. Passing `yargs` as positional arguments

```shell
foo_cmd | yargs 'basename {}' 'awk { print $2 }'
```

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
    yargs 'basename {}' "awk { print $1 }"
                                                    

    #OR
    yargs -f1 'basename {}' -f2 'awk { print $1 }'

would output: `ebook.pdf  |   Title`


- use colon as delimiter 

`yargs -d':' -f1 '...'`



---
[I am using Github under protest](protest.md)


