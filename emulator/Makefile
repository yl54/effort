# ------ General Commands ------

clean:
	rm -f test/*.o *.o emulator Makefile2

make-fmt:
	sed $'s/	/\t/g' Makefile > Makefile2; cat Makefile2 > Makefile;

# ------ Build Command ------

build: clean main.o opcode.o
	gcc -Wall -g -std=c11 -o emulator main.o opcode.o

# ------ Compiling Commands ------

main.o: main.c
    gcc -Wall -g -std=c11 -c main.c

opcode.o: opcode.c
	gcc -Wall -g -std=c11 -c opcode.c -o opcode.o

# ------ Test commands ------

test: opcode.o test/opcode.o

test-opcode:
	gcc -Wall -g -std=c11 -c test/opcode.c -o test/opcode.o
