	.text
	.file	"program"
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	format_str@GOTPCREL(%rip), %rdi
	movq	string_literal@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movl	$4, %edi
	callq	matematica_raiz_quadrada@PLT
	movq	%rax, %r14
	movl	$20, %edi
	callq	malloc@PLT
	movq	%rax, %rbx
	movq	int_format@GOTPCREL(%rip), %rsi
	movq	%rax, %rdi
	movq	%r14, %rdx
	xorl	%eax, %eax
	callq	sprintf@PLT
	movq	string_literal.1@GOTPCREL(%rip), %r14
	movq	%r14, %rdi
	callq	strlen@PLT
	movq	%rax, %r15
	movq	%rbx, %rdi
	callq	strlen@PLT
	leaq	1(%r15,%rax), %rdi
	callq	malloc@PLT
	movq	%rax, %r15
	movq	%rax, %rdi
	movq	%r14, %rsi
	callq	strcpy@PLT
	movq	%r15, %rdi
	movq	%rbx, %rsi
	callq	strcat@PLT
	movq	format_str.2@GOTPCREL(%rip), %rdi
	movq	%r15, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movl	$9, %edi
	callq	matematica_raiz_quadrada@PLT
	movq	%rax, %r14
	movl	$20, %edi
	callq	malloc@PLT
	movq	%rax, %rbx
	movq	int_format.4@GOTPCREL(%rip), %rsi
	movq	%rax, %rdi
	movq	%r14, %rdx
	xorl	%eax, %eax
	callq	sprintf@PLT
	movq	string_literal.3@GOTPCREL(%rip), %r14
	movq	%r14, %rdi
	callq	strlen@PLT
	movq	%rax, %r15
	movq	%rbx, %rdi
	callq	strlen@PLT
	leaq	1(%r15,%rax), %rdi
	callq	malloc@PLT
	movq	%rax, %r15
	movq	%rax, %rdi
	movq	%r14, %rsi
	callq	strcpy@PLT
	movq	%r15, %rdi
	movq	%rbx, %rsi
	callq	strcat@PLT
	movq	format_str.5@GOTPCREL(%rip), %rdi
	movq	%r15, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	xorl	%eax, %eax
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
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
	.asciz	"Testando apenas raiz quadrada:"
	.size	string_literal, 31

	.type	format_str,@object              # @format_str
	.globl	format_str
format_str:
	.asciz	"%s\n"
	.size	format_str, 4

	.type	string_literal.1,@object        # @string_literal.1
	.globl	string_literal.1
	.p2align	4, 0x0
string_literal.1:
	.asciz	"Raiz quadrada de 4: "
	.size	string_literal.1, 21

	.type	int_format,@object              # @int_format
	.globl	int_format
int_format:
	.asciz	"%lld"
	.size	int_format, 5

	.type	format_str.2,@object            # @format_str.2
	.globl	format_str.2
format_str.2:
	.asciz	"%s\n"
	.size	format_str.2, 4

	.type	string_literal.3,@object        # @string_literal.3
	.globl	string_literal.3
	.p2align	4, 0x0
string_literal.3:
	.asciz	"Raiz quadrada de 9: "
	.size	string_literal.3, 21

	.type	int_format.4,@object            # @int_format.4
	.globl	int_format.4
int_format.4:
	.asciz	"%lld"
	.size	int_format.4, 5

	.type	format_str.5,@object            # @format_str.5
	.globl	format_str.5
format_str.5:
	.asciz	"%s\n"
	.size	format_str.5, 4

	.section	".note.GNU-stack","",@progbits
