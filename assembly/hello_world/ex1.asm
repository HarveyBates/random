; ----------------------------------------------------------------------------------------
; Hello World program in assembly for MacOS x64
; To assemble and run:
;     nasm -f macho64 ex1.asm && ld ex1.o -o ex1.out -static && ./ex1.out
; ----------------------------------------------------------------------------------------

global start

section .data
	msg db "Hello, world!", 0x0a
	len equ $ - msg

section .text
start:
	mov rax, 0x02000004
	mov rdi, 1
	mov rsi, msg
	mov rdx, len
	syscall
	mov rax, 0x02000001
	mov rdi, 0
	syscall
