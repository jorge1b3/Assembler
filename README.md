# Assembler

This is an assembler program written in Rust that can parse assembly language files and convert them into binary machine code. To use the program, you will need to have Rust and Cargo installed on your system.

The assembler is designed to work with the Hack computer architecture, which is described in the book The Elements of Computing Systems by Noam Nisan and Shimon Schocken. The book is available online at https://www.nand2tetris.org/.

This is a solution to project 6 of the book, which can be found at https://www.nand2tetris.org/project06.

## Installation

To install Rust and Cargo, please follow the instructions on the official Rust website: https://www.rust-lang.org/tools/install.

## Usage

Once you have installed Rust and Cargo, you can clone the project repository and navigate to the project directory in your terminal. From there, you can run the following command to build the program:

```cargo build --release```

This will create an executable file in the target/release directory.

To use the program, you need to provide it with an assembly language file to parse. You can do this by running the following command:

```./target/release/assembler <path-to-asm-file>```

Replace `<path-to-asm-file>` with the path to the assembly language file you want to parse. This will parse the file and create a .hack file in the same directory as the assembly language file. The .hack file will have the same name as the assembly language file, but with the .hack extension.

## Contributing

This code is provided as-is and is not currently open to contributions.

## License

This project is licensed under the MIT License. See the LICENSE file for more information.
