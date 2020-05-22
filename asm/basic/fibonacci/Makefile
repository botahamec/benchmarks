
fibonacci: fibonacci.o
	gcc -m32 -o fibonacci fibonacci.o

fibonacci.o: fibonacci.asm
	nasm -f elf fibonacci.asm

clean:
	rm fibonacci fibonacci.o