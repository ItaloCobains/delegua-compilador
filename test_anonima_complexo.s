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
	subq	$16, %rsp
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	__anon_func_1@GOTPCREL(%rip), %rax
	movq	%rax, 8(%rsp)
	movq	__anon_func_2@GOTPCREL(%rip), %rcx
	movq	%rcx, (%rsp)
	movl	$10, %edi
	movl	$5, %esi
	callq	*%rax
	movq	%rax, %r14
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
	movl	$10, %edi
	movl	$5, %esi
	callq	*(%rsp)
	movq	%rax, %r14
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
	movl	$20, %edi
	movl	$3, %esi
	callq	*8(%rsp)
	movq	%rax, %r14
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
	addq	$16, %rsp
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
	.globl	__anon_func_1                   # -- Begin function __anon_func_1
	.p2align	4, 0x90
	.type	__anon_func_1,@function
__anon_func_1:                          # @__anon_func_1
	.cfi_startproc
# %bb.0:                                # %entry
	movq	%rdi, -8(%rsp)
	movq	%rsi, -16(%rsp)
	leaq	(%rdi,%rsi), %rax
	retq
.Lfunc_end1:
	.size	__anon_func_1, .Lfunc_end1-__anon_func_1
	.cfi_endproc
                                        # -- End function
	.globl	__anon_func_2                   # -- Begin function __anon_func_2
	.p2align	4, 0x90
	.type	__anon_func_2,@function
__anon_func_2:                          # @__anon_func_2
	.cfi_startproc
# %bb.0:                                # %entry
	movq	%rdi, %rax
	movq	%rdi, -8(%rsp)
	movq	%rsi, -16(%rsp)
	imulq	%rsi, %rax
	retq
.Lfunc_end2:
	.size	__anon_func_2, .Lfunc_end2-__anon_func_2
	.cfi_endproc
                                        # -- End function
	.type	string_literal,@object          # @string_literal
	.data
	.globl	string_literal
string_literal:
	.asciz	"Soma direta: "
	.size	string_literal, 14

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
	.p2align	4, 0x0
string_literal.1:
	.asciz	"Multiplica\303\247\303\243o direta: "
	.size	string_literal.1, 25

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
	.asciz	"Soma novamente: "
	.size	string_literal.4, 17

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
