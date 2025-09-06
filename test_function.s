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
	movl	$5, %edi
	callq	teste@PLT
	movq	%rax, %r14
	movq	%rax, 8(%rsp)
	movl	$20, %edi
	callq	malloc@PLT
	movq	%rax, %rbx
	movq	format_str@GOTPCREL(%rip), %rsi
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
	movq	format_str.1@GOTPCREL(%rip), %rdi
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
	.globl	teste                           # -- Begin function teste
	.p2align	4, 0x90
	.type	teste,@function
teste:                                  # @teste
	.cfi_startproc
# %bb.0:                                # %entry
	movq	%rdi, -8(%rsp)
	leaq	1(%rdi), %rax
	retq
.Lfunc_end1:
	.size	teste, .Lfunc_end1-teste
	.cfi_endproc
                                        # -- End function
	.type	string_literal,@object          # @string_literal
	.data
	.globl	string_literal
string_literal:
	.asciz	"resultado: "
	.size	string_literal, 12

	.type	format_str,@object              # @format_str
	.globl	format_str
format_str:
	.asciz	"%lld"
	.size	format_str, 5

	.type	format_str.1,@object            # @format_str.1
	.globl	format_str.1
format_str.1:
	.asciz	"%s\n"
	.size	format_str.1, 4

	.section	".note.GNU-stack","",@progbits
