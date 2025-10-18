	.build_version macos, 15, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	.cfi_def_cfa_offset 32
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh0:
	adrp	x0, _formato_texto@PAGE
Lloh1:
	add	x0, x0, _formato_texto@PAGEOFF
Lloh2:
	adrp	x8, _texto_literal@PAGE
Lloh3:
	add	x8, x8, _texto_literal@PAGEOFF
	str	x8, [sp]
	bl	_printf
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	mov	w0, wzr
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh0, Lloh1
	.cfi_endproc
                                        ; -- End function
	.section	__TEXT,__cstring,cstring_literals
	.globl	_texto_literal                  ; @texto_literal
_texto_literal:
	.asciz	"Ol\303\241, mundo!"

	.globl	_formato_texto                  ; @formato_texto
_formato_texto:
	.asciz	"%s\n"

.subsections_via_symbols
