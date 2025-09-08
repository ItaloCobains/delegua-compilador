	.text
	.file	"program"
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	$10, (%rsp)
	xorl	%eax, %eax
	testb	%al, %al
	jne	.LBB0_2
# %bb.1:                                # %then
	movq	format_str@GOTPCREL(%rip), %rdi
	movq	string_literal@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
.LBB0_2:                                # %merge
	cmpq	$4, (%rsp)
	jg	.LBB0_16
# %bb.3:                                # %then3
	movq	format_str@GOTPCREL(%rip), %rdi
	movq	string_literal.1@GOTPCREL(%rip), %rsi
	jmp	.LBB0_4
.LBB0_16:                               # %else
	movq	format_str@GOTPCREL(%rip), %rdi
	movq	string_literal.2@GOTPCREL(%rip), %rsi
.LBB0_4:                                # %merge4
	xorl	%eax, %eax
	callq	printf@PLT
	cmpq	$1, (%rsp)
	jne	.LBB0_14
# %bb.5:                                # %then8
	movq	format_str@GOTPCREL(%rip), %rdi
	movq	string_literal.3@GOTPCREL(%rip), %rsi
	jmp	.LBB0_6
.LBB0_14:                               # %else_if_start
	cmpq	$10, (%rsp)
	jne	.LBB0_15
# %bb.13:                               # %else_if_then_0
	movq	format_str@GOTPCREL(%rip), %rdi
	movq	string_literal.4@GOTPCREL(%rip), %rsi
	jmp	.LBB0_6
.LBB0_15:                               # %final_else
	movq	format_str@GOTPCREL(%rip), %rdi
	movq	string_literal.5@GOTPCREL(%rip), %rsi
.LBB0_6:                                # %merge9
	xorl	%eax, %eax
	callq	printf@PLT
	cmpq	$9, (%rsp)
	jg	.LBB0_7
# %bb.8:                                # %merge17
	cmpq	$15, (%rsp)
	jle	.LBB0_9
.LBB0_10:                               # %merge21
	cmpq	$5, (%rsp)
	je	.LBB0_12
.LBB0_11:                               # %then24
	movq	format_str@GOTPCREL(%rip), %rdi
	movq	string_literal.8@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
.LBB0_12:                               # %merge25
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.LBB0_7:                                # %then16
	.cfi_def_cfa_offset 16
	movq	format_str@GOTPCREL(%rip), %rdi
	movq	string_literal.6@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	cmpq	$15, (%rsp)
	jg	.LBB0_10
.LBB0_9:                                # %then20
	movq	format_str@GOTPCREL(%rip), %rdi
	movq	string_literal.7@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	cmpq	$5, (%rsp)
	jne	.LBB0_11
	jmp	.LBB0_12
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.globl	matematica_absoluto             # -- Begin function matematica_absoluto
	.p2align	4, 0x90
	.type	matematica_absoluto,@function
matematica_absoluto:                    # @matematica_absoluto
	.cfi_startproc
# %bb.0:                                # %entry
	movq	%rdi, %rax
	negq	%rax
	cmovsq	%rdi, %rax
	retq
.Lfunc_end1:
	.size	matematica_absoluto, .Lfunc_end1-matematica_absoluto
	.cfi_endproc
                                        # -- End function
	.globl	matematica_potencia             # -- Begin function matematica_potencia
	.p2align	4, 0x90
	.type	matematica_potencia,@function
matematica_potencia:                    # @matematica_potencia
	.cfi_startproc
# %bb.0:                                # %entry
	movq	$1, -16(%rsp)
	movq	%rsi, -8(%rsp)
	.p2align	4, 0x90
.LBB2_1:                                # %laco
                                        # =>This Inner Loop Header: Depth=1
	movq	-8(%rsp), %rax
	testq	%rax, %rax
	jle	.LBB2_3
# %bb.2:                                # %corpo_laco
                                        #   in Loop: Header=BB2_1 Depth=1
	movq	-16(%rsp), %rcx
	imulq	%rdi, %rcx
	movq	%rcx, -16(%rsp)
	decq	%rax
	movq	%rax, -8(%rsp)
	jmp	.LBB2_1
.LBB2_3:                                # %apos
	movq	-16(%rsp), %rax
	retq
.Lfunc_end2:
	.size	matematica_potencia, .Lfunc_end2-matematica_potencia
	.cfi_endproc
                                        # -- End function
	.globl	matematica_raiz_quadrada        # -- Begin function matematica_raiz_quadrada
	.p2align	4, 0x90
	.type	matematica_raiz_quadrada,@function
matematica_raiz_quadrada:               # @matematica_raiz_quadrada
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%rbx
	pushq	%rax
	.cfi_offset %rbx, -24
	testq	$-2, %rdi
	jne	.LBB3_2
# %bb.1:                                # %caso_especial
	movq	%rdi, %rax
	jmp	.LBB3_5
.LBB3_2:                                # %caso_normal
	movq	%rsp, %rax
	leaq	-16(%rax), %rcx
	movq	%rcx, %rsp
	movq	%rsp, %r8
	leaq	-16(%r8), %rdx
	movq	%rdx, %rsp
	movq	%rsp, %rsi
	addq	$-16, %rsi
	movq	%rsi, %rsp
	movq	$1, -16(%rax)
	movq	%rdi, -16(%r8)
	.p2align	4, 0x90
.LBB3_3:                                # %laco
                                        # =>This Inner Loop Header: Depth=1
	movq	(%rcx), %r8
	movq	(%rdx), %r9
	cmpq	%r9, %r8
	jg	.LBB3_6
# %bb.4:                                # %corpo_laco
                                        #   in Loop: Header=BB3_3 Depth=1
	leaq	(%r8,%r9), %r10
	movq	%r10, %rax
	shrq	$63, %rax
	addq	%r10, %rax
	sarq	%rax
	movq	%rax, %r10
	imulq	%rax, %r10
	cmpq	%rdi, %r10
	je	.LBB3_5
# %bb.7:                                # %continuar
                                        #   in Loop: Header=BB3_3 Depth=1
	leaq	-1(%rax), %r11
	leaq	1(%rax), %rbx
	cmpq	%rdi, %r10
	cmovgq	%r11, %r9
	cmovgq	%r8, %rbx
	movq	%r9, (%rdx)
	movq	%rbx, (%rcx)
	movq	%rax, (%rsi)
	jmp	.LBB3_3
.LBB3_6:                                # %apos_laco
	movq	(%rsi), %rax
.LBB3_5:                                # %retornar_meio
	leaq	-8(%rbp), %rsp
	popq	%rbx
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end3:
	.size	matematica_raiz_quadrada, .Lfunc_end3-matematica_raiz_quadrada
	.cfi_endproc
                                        # -- End function
	.type	string_literal,@object          # @string_literal
	.data
	.globl	string_literal
	.p2align	4, 0x0
string_literal:
	.asciz	"N\303\272mero \303\251 maior que 5"
	.size	string_literal, 23

	.type	format_str,@object              # @format_str
	.globl	format_str
format_str:
	.asciz	"%s\n"
	.size	format_str, 4

	.type	string_literal.1,@object        # @string_literal.1
	.globl	string_literal.1
	.p2align	4, 0x0
string_literal.1:
	.asciz	"N\303\272mero \303\251 menor que 5"
	.size	string_literal.1, 23

	.type	string_literal.2,@object        # @string_literal.2
	.globl	string_literal.2
	.p2align	4, 0x0
string_literal.2:
	.asciz	"N\303\272mero n\303\243o \303\251 menor que 5"
	.size	string_literal.2, 28

	.type	string_literal.3,@object        # @string_literal.3
	.globl	string_literal.3
string_literal.3:
	.asciz	"N\303\272mero \303\251 1"
	.size	string_literal.3, 13

	.type	string_literal.4,@object        # @string_literal.4
	.globl	string_literal.4
string_literal.4:
	.asciz	"N\303\272mero \303\251 10"
	.size	string_literal.4, 14

	.type	string_literal.5,@object        # @string_literal.5
	.globl	string_literal.5
	.p2align	4, 0x0
string_literal.5:
	.asciz	"N\303\272mero n\303\243o \303\251 1 nem 10"
	.size	string_literal.5, 25

	.type	string_literal.6,@object        # @string_literal.6
	.globl	string_literal.6
	.p2align	4, 0x0
string_literal.6:
	.asciz	"N\303\272mero \303\251 maior ou igual a 10"
	.size	string_literal.6, 31

	.type	string_literal.7,@object        # @string_literal.7
	.globl	string_literal.7
	.p2align	4, 0x0
string_literal.7:
	.asciz	"N\303\272mero \303\251 menor ou igual a 15"
	.size	string_literal.7, 31

	.type	string_literal.8,@object        # @string_literal.8
	.globl	string_literal.8
	.p2align	4, 0x0
string_literal.8:
	.asciz	"N\303\272mero \303\251 diferente de 5"
	.size	string_literal.8, 26

	.section	".note.GNU-stack","",@progbits
