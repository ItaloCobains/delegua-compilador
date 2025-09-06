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
	subq	$48, %rsp
	.cfi_def_cfa_offset 80
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	$10, 32(%rsp)
	movq	$4, (%rsp)
	movl	$20, %edi
	callq	malloc@PLT
	movq	%rax, %rbx
	movq	int_format@GOTPCREL(%rip), %rsi
	movl	$10, %edx
	movq	%rax, %rdi
	xorl	%eax, %eax
	callq	sprintf@PLT
	movq	string_literal@GOTPCREL(%rip), %r14
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
	movq	format_str@GOTPCREL(%rip), %rdi
	movq	%r15, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	(%rsp), %r14
	movl	$20, %edi
	callq	malloc@PLT
	movq	%rax, %rbx
	movq	int_format.2@GOTPCREL(%rip), %rsi
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
	movq	format_str.3@GOTPCREL(%rip), %rdi
	movq	%r15, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	32(%rsp), %rax
	movq	(%rsp), %rcx
	leaq	(%rax,%rcx), %r14
	movq	%rax, %rdx
	movq	%rax, %rsi
	subq	%rcx, %rsi
	movq	%rsi, 24(%rsp)
	movq	%r14, 40(%rsp)
	imulq	%rcx, %rdx
	movq	%rdx, 16(%rsp)
	cqto
	idivq	%rcx
	movq	%rax, 8(%rsp)
	movl	$20, %edi
	callq	malloc@PLT
	movq	%rax, %rbx
	movq	int_format.5@GOTPCREL(%rip), %rsi
	movq	%rax, %rdi
	movq	%r14, %rdx
	xorl	%eax, %eax
	callq	sprintf@PLT
	movq	string_literal.4@GOTPCREL(%rip), %r14
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
	movq	format_str.6@GOTPCREL(%rip), %rdi
	movq	%r15, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	24(%rsp), %r14
	movl	$20, %edi
	callq	malloc@PLT
	movq	%rax, %rbx
	movq	int_format.8@GOTPCREL(%rip), %rsi
	movq	%rax, %rdi
	movq	%r14, %rdx
	xorl	%eax, %eax
	callq	sprintf@PLT
	movq	string_literal.7@GOTPCREL(%rip), %r14
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
	movq	format_str.9@GOTPCREL(%rip), %rdi
	movq	%r15, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	16(%rsp), %r14
	movl	$20, %edi
	callq	malloc@PLT
	movq	%rax, %rbx
	movq	int_format.11@GOTPCREL(%rip), %rsi
	movq	%rax, %rdi
	movq	%r14, %rdx
	xorl	%eax, %eax
	callq	sprintf@PLT
	movq	string_literal.10@GOTPCREL(%rip), %r14
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
	movq	format_str.12@GOTPCREL(%rip), %rdi
	movq	%r15, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	8(%rsp), %r14
	movl	$20, %edi
	callq	malloc@PLT
	movq	%rax, %rbx
	movq	int_format.14@GOTPCREL(%rip), %rsi
	movq	%rax, %rdi
	movq	%r14, %rdx
	xorl	%eax, %eax
	callq	sprintf@PLT
	movq	string_literal.13@GOTPCREL(%rip), %r14
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
	movq	format_str.15@GOTPCREL(%rip), %rdi
	movq	%r15, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	xorl	%eax, %eax
	addq	$48, %rsp
	.cfi_def_cfa_offset 32
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
	.type	string_literal,@object          # @string_literal
	.data
	.globl	string_literal
string_literal:
	.asciz	"Valor de A: "
	.size	string_literal, 13

	.type	int_format,@object              # @int_format
	.globl	int_format
int_format:
	.asciz	"%lld"
	.size	int_format, 5

	.type	format_str,@object              # @format_str
	.globl	format_str
format_str:
	.asciz	"%s\n"
	.size	format_str, 4

	.type	string_literal.1,@object        # @string_literal.1
	.globl	string_literal.1
string_literal.1:
	.asciz	"Valor de B: "
	.size	string_literal.1, 13

	.type	int_format.2,@object            # @int_format.2
	.globl	int_format.2
int_format.2:
	.asciz	"%lld"
	.size	int_format.2, 5

	.type	format_str.3,@object            # @format_str.3
	.globl	format_str.3
format_str.3:
	.asciz	"%s\n"
	.size	format_str.3, 4

	.type	string_literal.4,@object        # @string_literal.4
	.globl	string_literal.4
	.p2align	4, 0x0
string_literal.4:
	.asciz	"A soma dos n\303\272meros \303\251 igual a: "
	.size	string_literal.4, 33

	.type	int_format.5,@object            # @int_format.5
	.globl	int_format.5
int_format.5:
	.asciz	"%lld"
	.size	int_format.5, 5

	.type	format_str.6,@object            # @format_str.6
	.globl	format_str.6
format_str.6:
	.asciz	"%s\n"
	.size	format_str.6, 4

	.type	string_literal.7,@object        # @string_literal.7
	.globl	string_literal.7
	.p2align	4, 0x0
string_literal.7:
	.asciz	"A subtra\303\247\303\243o dos n\303\272meros \303\251 igual a: "
	.size	string_literal.7, 40

	.type	int_format.8,@object            # @int_format.8
	.globl	int_format.8
int_format.8:
	.asciz	"%lld"
	.size	int_format.8, 5

	.type	format_str.9,@object            # @format_str.9
	.globl	format_str.9
format_str.9:
	.asciz	"%s\n"
	.size	format_str.9, 4

	.type	string_literal.10,@object       # @string_literal.10
	.globl	string_literal.10
	.p2align	4, 0x0
string_literal.10:
	.asciz	"A multiplica\303\247\303\243o dos n\303\272meros \303\251 igual a: "
	.size	string_literal.10, 44

	.type	int_format.11,@object           # @int_format.11
	.globl	int_format.11
int_format.11:
	.asciz	"%lld"
	.size	int_format.11, 5

	.type	format_str.12,@object           # @format_str.12
	.globl	format_str.12
format_str.12:
	.asciz	"%s\n"
	.size	format_str.12, 4

	.type	string_literal.13,@object       # @string_literal.13
	.globl	string_literal.13
	.p2align	4, 0x0
string_literal.13:
	.asciz	"A divis\303\243o dos n\303\272meros \303\251 igual a: "
	.size	string_literal.13, 37

	.type	int_format.14,@object           # @int_format.14
	.globl	int_format.14
int_format.14:
	.asciz	"%lld"
	.size	int_format.14, 5

	.type	format_str.15,@object           # @format_str.15
	.globl	format_str.15
format_str.15:
	.asciz	"%s\n"
	.size	format_str.15, 4

	.section	".note.GNU-stack","",@progbits
