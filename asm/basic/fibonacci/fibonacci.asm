NUM_OF_NUMBERS equ 50

section .text
	global _start

_start:

	mov eax, 1	; System exit with 0
	mov ebx, 0
	int 0x80