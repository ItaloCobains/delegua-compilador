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
	movq	$-5, 24(%rsp)
	movq	$-5, %rdi
	callq	math_abs@PLT
	movq	%rax, 16(%rsp)
	movl	$2, %edi
	movl	$3, %esi
	callq	math_pow@PLT
	movq	%rax, 8(%rsp)
	movl	$16, %edi
	callq	math_sqrt@PLT
	movq	%rax, (%rsp)
	movq	16(%rsp), %r14
	movl	$20, %edi
	callq	malloc@PLT
	movq	%rax, %rbx
	movq	int_format@GOTPCREL(%rip), %rsi
	movq	%rax, %rdi
	movq	%r14, %rdx
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
	movq	8(%rsp), %r14
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
	movq	(%rsp), %r14
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
	.globl	math_abs                        # -- Begin function math_abs
	.p2align	4, 0x90
	.type	math_abs,@function
math_abs:                               # @math_abs
	.cfi_startproc
# %bb.0:                                # %entry
	movq	%rdi, %rax
	negq	%rax
	cmovsq	%rdi, %rax
	retq
.Lfunc_end1:
	.size	math_abs, .Lfunc_end1-math_abs
	.cfi_endproc
                                        # -- End function
	.globl	math_pow                        # -- Begin function math_pow
	.p2align	4, 0x90
	.type	math_pow,@function
math_pow:                               # @math_pow
	.cfi_startproc
# %bb.0:                                # %entry
	movq	$1, -16(%rsp)
	movq	%rsi, -8(%rsp)
	.p2align	4, 0x90
.LBB2_1:                                # %loop
                                        # =>This Inner Loop Header: Depth=1
	movq	-8(%rsp), %rax
	testq	%rax, %rax
	jle	.LBB2_3
# %bb.2:                                # %loop_body
                                        #   in Loop: Header=BB2_1 Depth=1
	movq	-16(%rsp), %rcx
	imulq	%rdi, %rcx
	movq	%rcx, -16(%rsp)
	decq	%rax
	movq	%rax, -8(%rsp)
	jmp	.LBB2_1
.LBB2_3:                                # %after
	movq	-16(%rsp), %rax
	retq
.Lfunc_end2:
	.size	math_pow, .Lfunc_end2-math_pow
	.cfi_endproc
                                        # -- End function
	.globl	math_sqrt                       # -- Begin function math_sqrt
	.p2align	4, 0x90
	.type	math_sqrt,@function
math_sqrt:                              # @math_sqrt
	.cfi_startproc
# %bb.0:                                # %entry
	movl	$4, %eax
	retq
.Lfunc_end3:
	.size	math_sqrt, .Lfunc_end3-math_sqrt
	.cfi_endproc
                                        # -- End function
	.type	string_literal,@object          # @string_literal
	.data
	.globl	string_literal
string_literal:
	.asciz	"abs(-5) = "
	.size	string_literal, 11

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
	.asciz	"pow(2, 3) = "
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
string_literal.4:
	.asciz	"sqrt(16) = "
	.size	string_literal.4, 12

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

	.section	".note.GNU-stack","",@progbits
