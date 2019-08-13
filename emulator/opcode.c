// This file will cover opcodes.
// It will support the opcodes from the Intel 8080 microprocessor


// imports

// declarations

// q: does length of buffer have to be passed in as well?
// q: what failure scenarios might occur in these steps?
// q: do i return the coverted buffer, a success code, a pointer location, or nothing?


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

// function to disassemble one hex
// * input: a buffer of hex codes to convert. use unsigned char b/c its range is 0 - 255. Each char is one hex value.
// * pc   : a program counter to point to a hex value to analyze
int disassemble_hex_into_8080(unsigned char* input, int pc) {
    // switch case
    // do something with opcode
    // start with println
        // do stuff
        // ..
        // ..

    // return success code
    return 0;
}
