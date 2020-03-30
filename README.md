drawerrs
========

**_This project is work in progress._**

Draw ASCII art using simple commands (with rust).

Building
--------

The program needs [Rust](https://www.rust-lang.org/) and uses cargo to build.

To build it, execute:

    cargo build

There are tests that can be run with:

    cargo test

Running
-------

You can run it using cargo:

    cargo run

drawerrs stars in interactive mode waiting for commands.

Providing arguments `-r <filename>` allows to read instructions from a file.
It can also be run with cargo:

    cargo run -- -r <filename>

Commands
--------

All commands can be used both in interactive mode or when reading from a file.
The following commands are supported:

| Command           | Description
| ----------------- | -------------------------------
| CHAR ch           | Use `ch` character when drawing
| CANV width height | Create new canvas with the declared size
| CIRC x y r        | Draw a circle in point `(x, y)` with radius of the size `r`
| LINE x1 y1 x2 y2  | Draw a line from point `(x1, y1)` to point `(x2, y2)`
| RECT x1 y1 x2 y2  | Draw a rectangle from left top on `(x1, y1)` to right bottom on `(x2, y2)`
| INFO              | Display short info about canvas size
| SHOW              | Display the drawing on the screen
| READ filename     | Read and execute instructions from the provided file
| SAVE filename     | Save ASCII drawing to the provided file
| QUIT              | Quit program

License
=======

[MIT](https://opensource.org/licenses/MIT)
