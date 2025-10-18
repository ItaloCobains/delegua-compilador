	.build_version macos, 15, 0
	.section	__TEXT,__text,regular,pure_instructions
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
	mov	x0, #-5                         ; =0xfffffffffffffffb
	bl	_matematica_absoluto
	mov	x20, x0
	mov	w0, #20                         ; =0x14
	bl	_malloc
Lloh4:
	adrp	x1, _format_str.2@PAGE
Lloh5:
	add	x1, x1, _format_str.2@PAGEOFF
	mov	x21, x0
	str	x20, [sp]
	bl	_sprintf
Lloh6:
	adrp	x20, _string_literal.1@PAGE
Lloh7:
	add	x20, x20, _string_literal.1@PAGEOFF
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
	mov	w8, #3                          ; =0x3
	mov	w21, #1                         ; =0x1
LBB0_1:                                 ; %pow_loop
                                        ; =>This Inner Loop Header: Depth=1
	cmp	x8, #0
	stp	x8, x21, [sp, #16]
	b.le	LBB0_3
; %bb.2:                                ; %pow_multiply
                                        ;   in Loop: Header=BB0_1 Depth=1
	lsl	x21, x21, #1
	sub	x8, x8, #1
	b	LBB0_1
LBB0_3:                                 ; %pow_exit
	mov	w0, #20                         ; =0x14
	bl	_malloc
Lloh8:
	adrp	x19, _format_str.2@PAGE
Lloh9:
	add	x19, x19, _format_str.2@PAGEOFF
	mov	x20, x0
	mov	x1, x19
	str	x21, [sp]
	bl	_sprintf
Lloh10:
	adrp	x21, _string_literal.3@PAGE
Lloh11:
	add	x21, x21, _string_literal.3@PAGEOFF
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
Lloh12:
	adrp	x20, _format_str@PAGE
Lloh13:
	add	x20, x20, _format_str@PAGEOFF
	str	x22, [sp]
	mov	x0, x20
	bl	_printf
	mov	w0, #16                         ; =0x10
	bl	_matematica_raiz_quadrada
	mov	x21, x0
	mov	w0, #20                         ; =0x14
	bl	_malloc
	mov	x1, x19
	mov	x22, x0
	str	x21, [sp]
	bl	_sprintf
Lloh14:
	adrp	x21, _string_literal.4@PAGE
Lloh15:
	add	x21, x21, _string_literal.4@PAGEOFF
	mov	x0, x21
	bl	_strlen
	mov	x23, x0
	mov	x0, x22
	bl	_strlen
	add	x8, x23, x0
	add	x0, x8, #1
	bl	_malloc
	mov	x1, x21
	mov	x23, x0
	bl	_strcpy
	mov	x0, x23
	mov	x1, x22
	bl	_strcat
	mov	x0, x20
	str	x23, [sp]
	bl	_printf
	mov	w0, #25                         ; =0x19
	bl	_matematica_raiz_quadrada
	mov	x21, x0
	mov	w0, #20                         ; =0x14
	bl	_malloc
	mov	x1, x19
	mov	x22, x0
	str	x21, [sp]
	bl	_sprintf
Lloh16:
	adrp	x19, _string_literal.5@PAGE
Lloh17:
	add	x19, x19, _string_literal.5@PAGEOFF
	mov	x0, x19
	bl	_strlen
	mov	x21, x0
	mov	x0, x22
	bl	_strlen
	add	x8, x21, x0
	add	x0, x8, #1
	bl	_malloc
	mov	x1, x19
	mov	x21, x0
	bl	_strcpy
	mov	x0, x21
	mov	x1, x22
	bl	_strcat
	mov	x0, x20
	str	x21, [sp]
	bl	_printf
	ldp	x29, x30, [sp, #80]             ; 16-byte Folded Reload
	mov	x0, xzr
	ldp	x20, x19, [sp, #64]             ; 16-byte Folded Reload
	ldp	x22, x21, [sp, #48]             ; 16-byte Folded Reload
	ldp	x24, x23, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #96
	ret
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh0, Lloh1
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.cfi_endproc
                                        ; -- End function
	.globl	_matematica_absoluto            ; -- Begin function matematica_absoluto
	.p2align	2
_matematica_absoluto:                   ; @matematica_absoluto
	.cfi_startproc
; %bb.0:                                ; %entry
	cmp	x0, #0
	cneg	x0, x0, mi
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_matematica_raiz_quadrada       ; -- Begin function matematica_raiz_quadrada
	.p2align	2
_matematica_raiz_quadrada:              ; @matematica_raiz_quadrada
	.cfi_startproc
; %bb.0:                                ; %entry
	cmp	x0, #1
	b.ls	LBB2_8
; %bb.1:                                ; %caso_normal
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x11, sp
	sub	x8, x11, #16
	mov	sp, x8
	mov	x12, sp
	sub	x9, x12, #16
	mov	sp, x9
	sub	x10, sp, #16
	mov	sp, x10
	mov	w13, #1                         ; =0x1
	stur	x0, [x12, #-16]
	stur	x13, [x11, #-16]
LBB2_2:                                 ; %laco
                                        ; =>This Inner Loop Header: Depth=1
	ldr	x11, [x8]
	ldr	x12, [x9]
	cmp	x11, x12
	b.gt	LBB2_5
; %bb.3:                                ; %corpo_laco
                                        ;   in Loop: Header=BB2_2 Depth=1
	add	x13, x11, x12
	add	x13, x13, x13, lsr #63
	asr	x13, x13, #1
	mul	x14, x13, x13
	cmp	x14, x0
	b.eq	LBB2_6
; %bb.4:                                ; %continuar
                                        ;   in Loop: Header=BB2_2 Depth=1
	sub	x15, x13, #1
	cmp	x14, x0
	str	x13, [x10]
	csel	x12, x15, x12, gt
	csinc	x11, x11, x13, gt
	str	x12, [x9]
	str	x11, [x8]
	b	LBB2_2
LBB2_5:                                 ; %apos_laco
	ldr	x0, [x10]
	b	LBB2_7
LBB2_6:
	mov	x0, x13
LBB2_7:
	mov	sp, x29
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
LBB2_8:                                 ; %common.ret
	ret
	.cfi_endproc
                                        ; -- End function
	.section	__DATA,__data
	.globl	_string_literal                 ; @string_literal
	.p2align	4, 0x0
_string_literal:
	.asciz	"Testando fun\303\247\303\265es matem\303\241ticas:"

	.globl	_format_str                     ; @format_str
_format_str:
	.asciz	"%s\n"

	.globl	_string_literal.1               ; @string_literal.1
	.p2align	4, 0x0
_string_literal.1:
	.asciz	"Absoluto de -5: "

	.globl	_format_str.2                   ; @format_str.2
_format_str.2:
	.asciz	"%lld"

	.globl	_string_literal.3               ; @string_literal.3
	.p2align	4, 0x0
_string_literal.3:
	.asciz	"Pot\303\252ncia de 2^3: "

	.globl	_string_literal.4               ; @string_literal.4
	.p2align	4, 0x0
_string_literal.4:
	.asciz	"Raiz quadrada de 16: "

	.globl	_string_literal.5               ; @string_literal.5
	.p2align	4, 0x0
_string_literal.5:
	.asciz	"Raiz quadrada de 25: "

.subsections_via_symbols
