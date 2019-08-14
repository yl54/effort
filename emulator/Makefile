clean:
	rm -f *.o emulator Makefile2

make-fmt:
	sed $'s/	/\t/g' Makefile > Makefile2; cat Makefile2 > Makefile;

build: clean main.o opcode.o
	gcc -Wall -g -std=c11 -o emulator main.o opcode.o

main.o: main.c
    gcc -Wall -g -std=c11 -c main.c

opcode.o: opcode.c
	gcc -Wall -g -std=c11 -c opcode.c

test-opcode: opcode.c test-opcode.o

test-opcode.o:
	gcc -Wall -g -std=c11 -c test/opcode.c
