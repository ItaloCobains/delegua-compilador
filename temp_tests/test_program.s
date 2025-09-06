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
	subq	$32, %rsp
	.cfi_def_cfa_offset 64
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	$10, 24(%rsp)
	movq	$5, 16(%rsp)
	movq	$15, 8(%rsp)
	movl	$20, %edi
	callq	malloc@PLT
	movq	%rax, %rbx
	movq	int_format@GOTPCREL(%rip), %rsi
	movl	$15, %edx
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
	movq	string_literal.1@GOTPCREL(%rip), %rbx
	movq	%rbx, (%rsp)
	movq	string_literal.2@GOTPCREL(%rip), %r14
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
	movq	%r15, %rdi
	callq	strlen@PLT
	movq	%rax, %rbx
	movq	string_literal.3@GOTPCREL(%rip), %r14
	movq	%r14, %rdi
	callq	strlen@PLT
	leaq	1(%rbx,%rax), %rdi
	callq	malloc@PLT
	movq	%rax, %rbx
	movq	%rax, %rdi
	movq	%r15, %rsi
	callq	strcpy@PLT
	movq	%rbx, %rdi
	movq	%r14, %rsi
	callq	strcat@PLT
	movq	format_str.4@GOTPCREL(%rip), %rdi
	movq	%rbx, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	xorl	%eax, %eax
	addq	$32, %rsp
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
	.asciz	"A soma \303\251: "
	.size	string_literal, 12

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
	.asciz	"Delegua"
	.size	string_literal.1, 8

	.type	string_literal.2,@object        # @string_literal.2
	.globl	string_literal.2
string_literal.2:
	.asciz	"Ol\303\241 "
	.size	string_literal.2, 6

	.type	string_literal.3,@object        # @string_literal.3
	.globl	string_literal.3
string_literal.3:
	.asciz	"!"
	.size	string_literal.3, 2

	.type	format_str.4,@object            # @format_str.4
	.globl	format_str.4
format_str.4:
	.asciz	"%s\n"
	.size	format_str.4, 4

	.section	".note.GNU-stack","",@progbits
