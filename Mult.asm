// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// Put your code here.

    @i
    M=1
    @2
    M=0
(LOOP)
    @1
    D=M
    @0
    D=D-M
    @GREATER
    D;JGT
    @LESS
    0;JMP
(LESS)
    @i
    D=M
    @1
    D=D-M
    @END
    D;JGT

    @0
    D=M
    @2
    M=D+M

    @i
    M=M+1
    @LESS
    0;JMP
(GREATER)
    @i
    D=M
    @0
    D=D-M
    @END
    D;JGT

    @1
    D=M
    @2
    M=D+M

    @i
    M=M+1
    @GREATER
    0;JMP
(END)
    @END
    0;JMP