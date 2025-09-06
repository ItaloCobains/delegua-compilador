	.text
	.file	"program"
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	$0, -48(%rbp)
	movq	format_int@GOTPCREL(%rip), %rbx
	cmpq	$4, -48(%rbp)
	jg	.LBB0_3
	.p2align	4, 0x90
.LBB0_2:                                # %loop_body
                                        # =>This Inner Loop Header: Depth=1
	movq	-48(%rbp), %rsi
	movq	%rbx, %rdi
	xorl	%eax, %eax
	callq	printf@PLT
	incq	-48(%rbp)
	cmpq	$4, -48(%rbp)
	jle	.LBB0_2
.LBB0_3:                                # %after_loop
	movq	%rsp, %rax
	leaq	-16(%rax), %rbx
	movq	%rbx, %rsp
	movq	$0, -16(%rax)
	movq	string_literal@GOTPCREL(%rip), %r14
	movq	format_str@GOTPCREL(%rip), %r15
	cmpq	$4, (%rbx)
	jg	.LBB0_6
	.p2align	4, 0x90
.LBB0_5:                                # %for_body
                                        # =>This Inner Loop Header: Depth=1
	movq	(%rbx), %r13
	movl	$20, %edi
	callq	malloc@PLT
	movq	%rax, %r12
	movq	%rax, %rdi
	movq	int_format@GOTPCREL(%rip), %rsi
	movq	%r13, %rdx
	xorl	%eax, %eax
	callq	sprintf@PLT
	movq	%r14, %rdi
	callq	strlen@PLT
	movq	%rax, %r13
	movq	%r12, %rdi
	callq	strlen@PLT
	leaq	1(%r13,%rax), %rdi
	callq	malloc@PLT
	movq	%rax, %r13
	movq	%rax, %rdi
	movq	%r14, %rsi
	callq	strcpy@PLT
	movq	%r13, %rdi
	movq	%r12, %rsi
	callq	strcat@PLT
	movq	%r15, %rdi
	movq	%r13, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	incq	(%rbx)
	cmpq	$4, (%rbx)
	jle	.LBB0_5
.LBB0_6:                                # %after_for
	movq	format_str.2@GOTPCREL(%rip), %rdi
	movq	string_literal.1@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movq	$1, -16(%rax)
	movl	$20, %edi
	callq	malloc@PLT
	movq	%rax, %rbx
	movq	int_format.4@GOTPCREL(%rip), %rsi
	movl	$1, %edx
	movq	%rax, %rdi
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
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movq	$0, -16(%rax)
	movl	$20, %edi
	callq	malloc@PLT
	movq	%rax, %rbx
	movq	int_format.7@GOTPCREL(%rip), %rsi
	movq	%rax, %rdi
	xorl	%edx, %edx
	xorl	%eax, %eax
	callq	sprintf@PLT
	movq	string_literal.6@GOTPCREL(%rip), %r14
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
	movq	format_str.8@GOTPCREL(%rip), %rdi
	movq	%r15, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	xorl	%eax, %eax
	leaq	-40(%rbp), %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.type	format_int,@object              # @format_int
	.data
	.globl	format_int
format_int:
	.asciz	"%lld\n"
	.size	format_int, 6

	.type	string_literal,@object          # @string_literal
	.globl	string_literal
string_literal:
	.asciz	"For: "
	.size	string_literal, 6

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
	.asciz	"Do-while"
	.size	string_literal.1, 9

	.type	format_str.2,@object            # @format_str.2
	.globl	format_str.2
format_str.2:
	.asciz	"%s\n"
	.size	format_str.2, 4

	.type	string_literal.3,@object        # @string_literal.3
	.globl	string_literal.3
string_literal.3:
	.asciz	"Verdadeiro: "
	.size	string_literal.3, 13

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

	.type	string_literal.6,@object        # @string_literal.6
	.globl	string_literal.6
string_literal.6:
	.asciz	"Falso: "
	.size	string_literal.6, 8

	.type	int_format.7,@object            # @int_format.7
	.globl	int_format.7
int_format.7:
	.asciz	"%lld"
	.size	int_format.7, 5

	.type	format_str.8,@object            # @format_str.8
	.globl	format_str.8
format_str.8:
	.asciz	"%s\n"
	.size	format_str.8, 4

	.section	".note.GNU-stack","",@progbits
