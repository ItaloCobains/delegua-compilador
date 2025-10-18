	.build_version macos, 15, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
	.cfi_startproc
; %bb.0:                                ; %entry
	stp	x24, x23, [sp, #-64]!           ; 16-byte Folded Spill
	stp	x22, x21, [sp, #16]             ; 16-byte Folded Spill
	stp	x20, x19, [sp, #32]             ; 16-byte Folded Spill
	stp	x29, x30, [sp, #48]             ; 16-byte Folded Spill
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
Lloh0:
	adrp	x0, _texto_literal@PAGE
Lloh1:
	add	x0, x0, _texto_literal@PAGEOFF
	bl	_puts
	mov	w0, #256                        ; =0x100
	bl	_malloc
	cbz	x0, LBB0_13
; %bb.1:                                ; %alocacao_sucesso
	mov	x19, x0
Lloh2:
	adrp	x0, _formato_texto@PAGE
Lloh3:
	add	x0, x0, _formato_texto@PAGEOFF
	str	x19, [sp, #-16]!
	bl	_scanf
	add	sp, sp, #16
	mov	x8, sp
	sub	x9, x8, #16
	mov	sp, x9
	mov	x0, x19
	stur	x19, [x8, #-16]
	bl	_strlen
	add	x0, x0, #7
	bl	_malloc
	cbz	x0, LBB0_13
; %bb.2:                                ; %alocacao_sucesso3
Lloh4:
	adrp	x1, _texto_literal.1@PAGE
Lloh5:
	add	x1, x1, _texto_literal.1@PAGEOFF
	mov	x20, x0
	bl	_strcpy
	mov	x0, x20
	mov	x1, x19
	bl	_strcat
	mov	x0, x20
	bl	_strlen
	add	x0, x0, #2
	bl	_malloc
	cbz	x0, LBB0_13
; %bb.3:                                ; %alocacao_sucesso10
	mov	x1, x20
	mov	x21, x0
	bl	_strcpy
Lloh6:
	adrp	x1, _texto_literal.2@PAGE
Lloh7:
	add	x1, x1, _texto_literal.2@PAGEOFF
	mov	x0, x21
	bl	_strcat
Lloh8:
	adrp	x0, _formato_texto.3@PAGE
Lloh9:
	add	x0, x0, _formato_texto.3@PAGEOFF
	str	x21, [sp, #-16]!
	bl	_printf
	add	sp, sp, #16
	mov	x0, x20
	bl	_free
	mov	x0, x21
	bl	_free
Lloh10:
	adrp	x0, _texto_literal.4@PAGE
Lloh11:
	add	x0, x0, _texto_literal.4@PAGEOFF
	bl	_puts
	mov	w0, #256                        ; =0x100
	bl	_malloc
	cbz	x0, LBB0_13
; %bb.4:                                ; %alocacao_sucesso19
	mov	x20, x0
Lloh12:
	adrp	x0, _formato_texto@PAGE
Lloh13:
	add	x0, x0, _formato_texto@PAGEOFF
	str	x20, [sp, #-16]!
	bl	_scanf
	add	sp, sp, #16
	mov	x8, sp
	sub	x9, x8, #16
	mov	sp, x9
	mov	x0, x20
	stur	x20, [x8, #-16]
	bl	_strlen
	add	x0, x0, #11
	bl	_malloc
	cbz	x0, LBB0_13
; %bb.5:                                ; %alocacao_sucesso29
Lloh14:
	adrp	x1, _texto_literal.5@PAGE
Lloh15:
	add	x1, x1, _texto_literal.5@PAGEOFF
	mov	x21, x0
	bl	_strcpy
	mov	x0, x21
	mov	x1, x20
	bl	_strcat
	mov	x0, x21
	bl	_strlen
	add	x0, x0, #7
	bl	_malloc
	cbz	x0, LBB0_13
; %bb.6:                                ; %alocacao_sucesso39
	mov	x1, x21
	mov	x22, x0
	bl	_strcpy
Lloh16:
	adrp	x1, _texto_literal.6@PAGE
Lloh17:
	add	x1, x1, _texto_literal.6@PAGEOFF
	mov	x0, x22
	bl	_strcat
Lloh18:
	adrp	x0, _formato_texto.3@PAGE
Lloh19:
	add	x0, x0, _formato_texto.3@PAGEOFF
	str	x22, [sp, #-16]!
	bl	_printf
	add	sp, sp, #16
	mov	x0, x21
	bl	_free
	mov	x0, x22
	bl	_free
Lloh20:
	adrp	x0, _texto_literal.7@PAGE
Lloh21:
	add	x0, x0, _texto_literal.7@PAGEOFF
	bl	_puts
	mov	w0, #256                        ; =0x100
	bl	_malloc
	cbz	x0, LBB0_13
; %bb.7:                                ; %alocacao_sucesso50
	mov	x21, x0
Lloh22:
	adrp	x0, _formato_texto@PAGE
Lloh23:
	add	x0, x0, _formato_texto@PAGEOFF
	str	x21, [sp, #-16]!
	bl	_scanf
	add	sp, sp, #16
	mov	x8, sp
	sub	x9, x8, #16
	mov	sp, x9
	mov	x0, x21
	stur	x21, [x8, #-16]
	bl	_strlen
	add	x0, x0, #15
	bl	_malloc
	cbz	x0, LBB0_13
; %bb.8:                                ; %alocacao_sucesso60
Lloh24:
	adrp	x1, _texto_literal.8@PAGE
Lloh25:
	add	x1, x1, _texto_literal.8@PAGEOFF
	mov	x22, x0
	bl	_strcpy
	mov	x0, x22
	mov	x1, x21
	bl	_strcat
	mov	x0, x22
	bl	_strlen
	add	x0, x0, #9
	bl	_malloc
	cbz	x0, LBB0_13
; %bb.9:                                ; %alocacao_sucesso70
	mov	x1, x22
	mov	x23, x0
	bl	_strcpy
Lloh26:
	adrp	x1, _texto_literal.9@PAGE
Lloh27:
	add	x1, x1, _texto_literal.9@PAGEOFF
	mov	x0, x23
	bl	_strcat
Lloh28:
	adrp	x0, _formato_texto.3@PAGE
Lloh29:
	add	x0, x0, _formato_texto.3@PAGEOFF
	str	x23, [sp, #-16]!
	bl	_printf
	add	sp, sp, #16
	mov	x0, x22
	bl	_free
	mov	x0, x23
	bl	_free
Lloh30:
	adrp	x0, _texto_literal.10@PAGE
Lloh31:
	add	x0, x0, _texto_literal.10@PAGEOFF
	bl	_puts
	mov	w0, #256                        ; =0x100
	bl	_malloc
	cbz	x0, LBB0_13
; %bb.10:                               ; %alocacao_sucesso81
	mov	x22, x0
Lloh32:
	adrp	x0, _formato_texto@PAGE
Lloh33:
	add	x0, x0, _formato_texto@PAGEOFF
	str	x22, [sp, #-16]!
	bl	_scanf
	add	sp, sp, #16
	mov	x8, sp
	sub	x9, x8, #16
	mov	sp, x9
	mov	x0, x22
	stur	x22, [x8, #-16]
	bl	_strlen
	add	x0, x0, #13
	bl	_malloc
	cbz	x0, LBB0_13
; %bb.11:                               ; %alocacao_sucesso91
Lloh34:
	adrp	x1, _texto_literal.11@PAGE
Lloh35:
	add	x1, x1, _texto_literal.11@PAGEOFF
	mov	x23, x0
	bl	_strcpy
	mov	x0, x23
	mov	x1, x22
	bl	_strcat
	mov	x0, x23
	bl	_strlen
	add	x0, x0, #5
	bl	_malloc
	cbz	x0, LBB0_13
; %bb.12:                               ; %alocacao_sucesso101
	mov	x1, x23
	mov	x24, x0
	bl	_strcpy
Lloh36:
	adrp	x1, _texto_literal.12@PAGE
Lloh37:
	add	x1, x1, _texto_literal.12@PAGEOFF
	mov	x0, x24
	bl	_strcat
Lloh38:
	adrp	x0, _formato_texto.3@PAGE
Lloh39:
	add	x0, x0, _formato_texto.3@PAGEOFF
	str	x24, [sp, #-16]!
	bl	_printf
	add	sp, sp, #16
	mov	x0, x23
	bl	_free
	mov	x0, x24
	bl	_free
	mov	x0, x19
	bl	_free
	mov	x0, x20
	bl	_free
	mov	x0, x21
	bl	_free
	mov	x0, x22
	bl	_free
	mov	w0, wzr
	sub	sp, x29, #48
	ldp	x29, x30, [sp, #48]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp, #32]             ; 16-byte Folded Reload
	ldp	x22, x21, [sp, #16]             ; 16-byte Folded Reload
	ldp	x24, x23, [sp], #64             ; 16-byte Folded Reload
	ret
LBB0_13:                                ; %alocacao_falhou
	mov	w0, #1                          ; =0x1
	bl	_exit
	brk	#0x1
	.loh AdrpAdd	Lloh0, Lloh1
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh38, Lloh39
	.loh AdrpAdd	Lloh36, Lloh37
	.cfi_endproc
                                        ; -- End function
	.section	__TEXT,__cstring,cstring_literals
	.globl	_texto_literal                  ; @texto_literal
	.p2align	4, 0x0
_texto_literal:
	.asciz	"Digite seu nome: "

	.globl	_formato_texto                  ; @formato_texto
_formato_texto:
	.asciz	"%255s"

	.globl	_texto_literal.1                ; @texto_literal.1
_texto_literal.1:
	.asciz	"Ol\303\241, "

	.globl	_texto_literal.2                ; @texto_literal.2
_texto_literal.2:
	.asciz	"!"

	.globl	_formato_texto.3                ; @formato_texto.3
_formato_texto.3:
	.asciz	"%s\n"

	.globl	_texto_literal.4                ; @texto_literal.4
	.p2align	4, 0x0
_texto_literal.4:
	.asciz	"Digite sua idade: "

	.globl	_texto_literal.5                ; @texto_literal.5
_texto_literal.5:
	.asciz	"Voc\303\252 tem "

	.globl	_texto_literal.6                ; @texto_literal.6
_texto_literal.6:
	.asciz	" anos."

	.globl	_texto_literal.7                ; @texto_literal.7
	.p2align	4, 0x0
_texto_literal.7:
	.asciz	"Digite sua altura em metros: "

	.globl	_texto_literal.8                ; @texto_literal.8
_texto_literal.8:
	.asciz	"Sua altura \303\251 "

	.globl	_texto_literal.9                ; @texto_literal.9
_texto_literal.9:
	.asciz	" metros."

	.globl	_texto_literal.10               ; @texto_literal.10
	.p2align	4, 0x0
_texto_literal.10:
	.asciz	"Digite seu peso em kg: "

	.globl	_texto_literal.11               ; @texto_literal.11
_texto_literal.11:
	.asciz	"Seu peso \303\251 "

	.globl	_texto_literal.12               ; @texto_literal.12
_texto_literal.12:
	.asciz	" kg."

.subsections_via_symbols
