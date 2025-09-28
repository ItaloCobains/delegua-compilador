	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 15, 0
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #96
	stp	x24, x23, [sp, #32]             ; 16-byte Folded Spill
	stp	x22, x21, [sp, #48]             ; 16-byte Folded Spill
	stp	x20, x19, [sp, #64]             ; 16-byte Folded Spill
	stp	x29, x30, [sp, #80]             ; 16-byte Folded Spill
	.cfi_def_cfa_offset 96
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
Lloh0:
	adrp	x19, _format_str@PAGE
Lloh1:
	add	x19, x19, _format_str@PAGEOFF
Lloh2:
	adrp	x8, _string_literal@PAGE
Lloh3:
	add	x8, x8, _string_literal@PAGEOFF
	mov	x0, x19
	str	x8, [sp]
	bl	_printf
Lloh4:
	adrp	x20, _format_str.2@PAGE
Lloh5:
	add	x20, x20, _format_str.2@PAGEOFF
Lloh6:
	adrp	x8, _string_literal.1@PAGE
Lloh7:
	add	x8, x8, _string_literal.1@PAGEOFF
	mov	x0, x20
	str	x8, [sp]
	bl	_printf
	mov	w0, #256                        ; =0x100
	bl	_malloc
Lloh8:
	adrp	x21, _format_str.3@PAGE
Lloh9:
	add	x21, x21, _format_str.3@PAGEOFF
	mov	x22, x0
	mov	x0, x21
	str	x22, [sp]
	bl	_scanf
Lloh10:
	adrp	x23, _string_literal.4@PAGE
Lloh11:
	add	x23, x23, _string_literal.4@PAGEOFF
	str	x22, [sp, #24]
	mov	x0, x23
	bl	_strlen
	mov	x24, x0
	mov	x0, x22
	bl	_strlen
	add	x8, x24, x0
	add	x0, x8, #1
	bl	_malloc
	mov	x1, x23
	mov	x24, x0
	bl	_strcpy
	mov	x0, x24
	mov	x1, x22
	bl	_strcat
	mov	x0, x24
	bl	_strlen
Lloh12:
	adrp	x23, _string_literal.5@PAGE
Lloh13:
	add	x23, x23, _string_literal.5@PAGEOFF
	mov	x22, x0
	mov	x0, x23
	bl	_strlen
	add	x8, x22, x0
	add	x0, x8, #1
	bl	_malloc
	mov	x1, x24
	mov	x22, x0
	bl	_strcpy
	mov	x0, x22
	mov	x1, x23
	bl	_strcat
	mov	x0, x19
	str	x22, [sp]
	bl	_printf
	mov	x0, x20
Lloh14:
	adrp	x8, _string_literal.6@PAGE
Lloh15:
	add	x8, x8, _string_literal.6@PAGEOFF
	str	x8, [sp]
	bl	_printf
	mov	w0, #256                        ; =0x100
	bl	_malloc
	mov	x20, x0
	mov	x0, x21
	str	x20, [sp]
	bl	_scanf
Lloh16:
	adrp	x21, _string_literal.7@PAGE
Lloh17:
	add	x21, x21, _string_literal.7@PAGEOFF
	str	x20, [sp, #16]
	mov	x0, x21
	bl	_strlen
	mov	x22, x0
	mov	x0, x20
	bl	_strlen
	add	x8, x22, x0
	add	x0, x8, #1
	bl	_malloc
	mov	x1, x21
	mov	x22, x0
	bl	_strcpy
	mov	x0, x22
	mov	x1, x20
	bl	_strcat
	mov	x0, x19
	str	x22, [sp]
	bl	_printf
	ldp	x29, x30, [sp, #80]             ; 16-byte Folded Reload
	mov	x0, xzr
	ldp	x20, x19, [sp, #64]             ; 16-byte Folded Reload
	ldp	x22, x21, [sp, #48]             ; 16-byte Folded Reload
	ldp	x24, x23, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #96
	ret
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh0, Lloh1
	.cfi_endproc
                                        ; -- End function
	.section	__DATA,__data
	.globl	_string_literal                 ; @string_literal
	.p2align	4, 0x0
_string_literal:
	.asciz	"Teste da fun\303\247\303\243o leia:"

	.globl	_format_str                     ; @format_str
_format_str:
	.asciz	"%s\n"

	.globl	_string_literal.1               ; @string_literal.1
	.p2align	4, 0x0
_string_literal.1:
	.asciz	"Digite seu nome: "

	.globl	_format_str.2                   ; @format_str.2
_format_str.2:
	.asciz	"%s"

	.globl	_format_str.3                   ; @format_str.3
_format_str.3:
	.asciz	"%255s"

	.globl	_string_literal.4               ; @string_literal.4
_string_literal.4:
	.asciz	"Ol\303\241 "

	.globl	_string_literal.5               ; @string_literal.5
_string_literal.5:
	.asciz	"!"

	.globl	_string_literal.6               ; @string_literal.6
	.p2align	4, 0x0
_string_literal.6:
	.asciz	"Digite sua idade: "

	.globl	_string_literal.7               ; @string_literal.7
_string_literal.7:
	.asciz	"Voc\303\252 digitou: "

.subsections_via_symbols
