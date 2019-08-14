// This file will cover opcodes.
// It will support the opcodes from the Intel 8080 microprocessor


// imports

// declarations
#include <stdio.h>

// q: does length of buffer have to be passed in as well?
// q: what failure scenarios might occur in these steps?
// q: do i return the coverted buffer, a success code, a pointer location, or nothing?

// Map of int to hex conversions, its really an array
// key: index
// value: opcode

// list of opcodes

// disassemble_8080 converts a buffer of hex codes to a buffer of 8080 opcodes
// * input: a buffer of hex codes to convert. use unsigned char b/c its range is 0 - 255. Each char is one hex value.
int disassemble_buffer_into_8080(unsigned char* input) {
    // initialize a pc to count through everything

    // while loop, check if pointer is pointing to buffer
        // pass into dissasemble hex function

        // move pointer to next hex value

    // return success code
    return 0;
}

// disassemble_hex_into_8080 disassembles one hex code into an opcode
// return number of bytes read from input buffer
// * input: a buffer of hex codes to convert. use unsigned char b/c its range is 0 - 255. Each char is one hex value.
// * pc: a program counter to point to a hex value to analyze
int disassemble_hex_into_8080(unsigned char* input, int pc) {
    // switch case
    // do something with opcode
    // start with println
    unsigned char* code = &input[pc];

    // init opcode to invalid case
    int pc_read = -1;
    switch (*code) {
        // do stuff

        case 0x00:
        {
            fprintf(stdout, "NOP");
            opcode = 1;
            break;
        }
        case 0x01:
        {
            unsigned char* code_1 = &code[1];
            unsigned char* code_2 = &code[2];
            fprintf(stdout, "LXI    B,#$%02x%02x", *code_1, *code_2);
            break;
        }
        case 0x02:
        {
            fprintf(stdout, "LXI    B");
            break;
        }
        case 0x03:
        {
            fprintf(stdout, "INX    B");
            break;
        }
        case 0x04:
        {
            fprintf(stdout, "INR    B");
            break;
        }
        case 0x05:
        {
            fprintf(stdout, "DCR    B");
            break;
        }
        // ..
        // ..
    }

    fprintf(stdout, "\n");

    // return a failure scenario
    return 0;
}
