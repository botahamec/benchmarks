NUM_OF_NUMBERS equ 40

section .text
	global main
	extern printf

	main:
		mov eax, 0				; a = 0
		mov ebx, 1				; b = 1
		mov edx, 0				; c = 0

		mov ecx, NUM_OF_NUMBERS ; let i = NUM_OF_NUMBERS

	one_num:
		jcxz end				; while (i > 0)

		mov edx, eax			; c = a
		add edx, ebx			; c += b

		mov eax, ebx			; a = b
		mov ebx, edx			; b = c

		mov [vara], eax			; store values before printf call
		mov [varb], ebx
		mov [varc], ecx
		mov [current], ecx

		push edx
		push line
		call printf				; printf("%d", c)
		add esp, 8

		mov eax, [vara]			; get values back
		mov ebx, [varb]
		mov edx, [varb]
		mov ecx, [current]

		dec ecx					; i --
		jmp one_num				; goto one_num

	end:
		ret

section .data
	line db "%d", 10, 0

section .bss
	vara:		resb	4
	varb:		resb	4
	varc:		resb	4
	current:	resb	4