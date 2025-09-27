	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 15, 0
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #112
	stp	x24, x23, [sp, #48]             ; 16-byte Folded Spill
	stp	x22, x21, [sp, #64]             ; 16-byte Folded Spill
	stp	x20, x19, [sp, #80]             ; 16-byte Folded Spill
	stp	x29, x30, [sp, #96]             ; 16-byte Folded Spill
	.cfi_def_cfa_offset 112
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	mov	w8, #1                          ; =0x1
Lloh0:
	adrp	x19, _format_str@PAGE
Lloh1:
	add	x19, x19, _format_str@PAGEOFF
	stp	xzr, x8, [sp, #32]
	mov	x0, x19
	stp	x8, xzr, [sp, #16]
Lloh2:
	adrp	x8, _string_literal@PAGE
Lloh3:
	add	x8, x8, _string_literal@PAGEOFF
	str	x8, [sp]
	bl	_printf
	ldr	x22, [sp, #40]
	mov	w0, #20                         ; =0x14
	bl	_malloc
Lloh4:
	adrp	x20, _format_str.2@PAGE
Lloh5:
	add	x20, x20, _format_str.2@PAGEOFF
	mov	x21, x0
	mov	x1, x20
	str	x22, [sp]
	bl	_sprintf
Lloh6:
	adrp	x22, _string_literal.1@PAGE
Lloh7:
	add	x22, x22, _string_literal.1@PAGEOFF
	mov	x0, x22
	bl	_strlen
	mov	x23, x0
	mov	x0, x21
	bl	_strlen
	add	x8, x23, x0
	add	x0, x8, #1
	bl	_malloc
	mov	x1, x22
	mov	x23, x0
	bl	_strcpy
	mov	x0, x23
	mov	x1, x21
	bl	_strcat
	mov	x0, x19
	str	x23, [sp]
	bl	_printf
	ldr	x22, [sp, #32]
	mov	w0, #20                         ; =0x14
	bl	_malloc
	mov	x1, x20
	mov	x21, x0
	str	x22, [sp]
	bl	_sprintf
Lloh8:
	adrp	x22, _string_literal.3@PAGE
Lloh9:
	add	x22, x22, _string_literal.3@PAGEOFF
	mov	x0, x22
	bl	_strlen
	mov	x23, x0
	mov	x0, x21
	bl	_strlen
	add	x8, x23, x0
	add	x0, x8, #1
	bl	_malloc
	mov	x1, x22
	mov	x23, x0
	bl	_strcpy
	mov	x0, x23
	mov	x1, x21
	bl	_strcat
	mov	x0, x19
	str	x23, [sp]
	bl	_printf
	ldr	x22, [sp, #24]
	mov	w0, #20                         ; =0x14
	bl	_malloc
	mov	x1, x20
	mov	x21, x0
	str	x22, [sp]
	bl	_sprintf
Lloh10:
	adrp	x22, _string_literal.4@PAGE
Lloh11:
	add	x22, x22, _string_literal.4@PAGEOFF
	mov	x0, x22
	bl	_strlen
	mov	x23, x0
	mov	x0, x21
	bl	_strlen
	add	x8, x23, x0
	add	x0, x8, #1
	bl	_malloc
	mov	x1, x22
	mov	x23, x0
	bl	_strcpy
	mov	x0, x23
	mov	x1, x21
	bl	_strcat
	mov	x0, x19
	str	x23, [sp]
	bl	_printf
	ldr	x22, [sp, #16]
	mov	w0, #20                         ; =0x14
	bl	_malloc
	mov	x1, x20
	mov	x21, x0
	str	x22, [sp]
	bl	_sprintf
Lloh12:
	adrp	x20, _string_literal.5@PAGE
Lloh13:
	add	x20, x20, _string_literal.5@PAGEOFF
	mov	x0, x20
	bl	_strlen
	mov	x22, x0
	mov	x0, x21
	bl	_strlen
	add	x8, x22, x0
	add	x0, x8, #1
	bl	_malloc
	mov	x1, x20
	mov	x22, x0
	bl	_strcpy
	mov	x0, x22
	mov	x1, x21
	bl	_strcat
	mov	x0, x19
	str	x22, [sp]
	bl	_printf
	ldp	x29, x30, [sp, #96]             ; 16-byte Folded Reload
	mov	x0, xzr
	ldp	x20, x19, [sp, #80]             ; 16-byte Folded Reload
	ldp	x22, x21, [sp, #64]             ; 16-byte Folded Reload
	ldp	x24, x23, [sp, #48]             ; 16-byte Folded Reload
	add	sp, sp, #112
	ret
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
	.asciz	"Teste de operadores l\303\263gicos:"

	.globl	_format_str                     ; @format_str
_format_str:
	.asciz	"%s\n"

	.globl	_string_literal.1               ; @string_literal.1
_string_literal.1:
	.asciz	"a = "

	.globl	_format_str.2                   ; @format_str.2
_format_str.2:
	.asciz	"%lld"

	.globl	_string_literal.3               ; @string_literal.3
_string_literal.3:
	.asciz	"b = "

	.globl	_string_literal.4               ; @string_literal.4
_string_literal.4:
	.asciz	"a e b = "

	.globl	_string_literal.5               ; @string_literal.5
_string_literal.5:
	.asciz	"a ou b = "

.subsections_via_symbols
