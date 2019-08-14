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

int disassemble_buffer_into_8080(unsigned char* input, int size);
int disassemble_hex_into_8080(unsigned char* input, int pc);

// disassemble_8080 converts a buffer of hex codes to a buffer of 8080 opcodes
// * input: a buffer of hex codes to convert. use unsigned char b/c its range is 0 - 255. Each char is one hex value.
int disassemble_buffer_into_8080(unsigned char* input, int size) {
    // initialize a pc to count through everything
    int pc = 0;

    fprintf(stdout, "Start reading buffer: \n");

    // while loop, check if pointer is less than a size
    while (pc < size) {

        // pass into dissasemble hex function
        int bytes_read = disassemble_hex_into_8080(input, pc);

        if (bytes_read == -1) {
            // stop this from continuing
            // this is a bad outcome
            return -1;
        }

        // move pointer to next hex value
        pc += bytes_read;
    }

    fprintf(stdout, "Finish reading buffer. \n");

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
            pc_read = 1;
            break;
        }
        case 0x01:
        {
            unsigned char* code_1 = &code[1];
            unsigned char* code_2 = &code[2];
            fprintf(stdout, "LXI    B,#$%02x%02x", *code_1, *code_2);
            pc_read = 3;
            break;
        }
        case 0x02:
        {
            fprintf(stdout, "LXI    B");
            pc_read = 1;
            break;
        }
        case 0x03:
        {
            fprintf(stdout, "INX    B");
            pc_read = 1;
            break;
        }
        case 0x04:
        {
            fprintf(stdout, "INR    B");
            pc_read = 1;
            break;
        }
        case 0x05:
        {
            fprintf(stdout, "DCR    B");
            pc_read = 1;
            break;
        }
        // ..
        // ..
    }

    fprintf(stdout, "\n");

    // return a failure scenario
    return pc_read;
}
