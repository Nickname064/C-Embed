# C-embed
A really simple command to embed a binary file into a C program

## Why
One of the things that can be annoying when working in C, compared to Rust, is the absence of `include_bytes!`,
which can really be useful when you need to bake data.
This tool aims to provide a decent way to do this.

## How
The program will read from the standard input, and write to the standard input a C array containing the binary data.
If you include that array in one of your C programs, it will be embedded into the executable.

## Notes
The tool is voluntarily very simple so that it can be combined with other tools/scripts.
If you just really need to turn one file into a C header, just do
```cat [your_file] | c-embed [c_array_name] > header.h```