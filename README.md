# Redirect

A small utility that makes it possible to redirect IO from files where such redirection is not supported.

## Example

The following `shell` code:

```sh
/bin/foo arg1 arg2 -f opt1 -abc < input.txt > output.txt
```
can be written with `redirect` as:

```sh
redirect /bin/foo -i input.txt -o output.txt -- arg1 arg2 -f opt1 -abc
```

## Use Case

1. Task runners that don't directly support shell redirections
    e.g.: VScode tasks
