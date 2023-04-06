# rxd

hex file dumper written in Rust

```
Usage: rxd [OPTIONS] <FILE_PATH>

Arguments:
  <FILE_PATH>  input file

Options:
  -l <LINE_COUNT>             number of lines to print
  -w <LINE_WIDTH>             number of bytes per line [default: 16]
  -g <BYTE_GROUP_LENGTH>      number of bytes grouped together per line [default: 1]
  -c                          display C0 control codes as characters
  -h, --help                  Print help
  -V, --version               Print version
```
