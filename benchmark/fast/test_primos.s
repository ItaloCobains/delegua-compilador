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
	movq	$1000000, 8(%rsp)               # imm = 0xF4240
	movl	$1000000, %edi                  # imm = 0xF4240
	callq	contarPrimos@PLT
	movq	%rax, %r14
	movq	%rax, (%rsp)
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
	.globl	ehPrimo                         # -- Begin function ehPrimo
	.p2align	4, 0x90
	.type	ehPrimo,@function
ehPrimo:                                # @ehPrimo
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	%rdi, -8(%rbp)
	cmpq	$1, %rdi
	jg	.LBB1_3
.LBB1_1:                                # %then
	xorl	%eax, %eax
	jmp	.LBB1_2
.LBB1_3:                                # %merge
	cmpq	$2, -8(%rbp)
	jne	.LBB1_5
.LBB1_4:                                # %then3
	movl	$1, %eax
.LBB1_2:                                # %then
	movq	%rbp, %rsp
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB1_5:                                # %merge4
	.cfi_def_cfa %rbp, 16
	movq	-8(%rbp), %rax
	movq	%rax, %rcx
	shrq	$63, %rcx
	addq	%rax, %rcx
	andq	$-2, %rcx
	cmpq	%rcx, %rax
	je	.LBB1_1
# %bb.6:                                # %merge8
	movq	-8(%rbp), %rax
	movq	%rax, %rdx
	shrq	$63, %rdx
	addq	%rax, %rdx
	sarq	%rdx
	movq	%rsp, %rax
	leaq	-16(%rax), %rcx
	movq	%rcx, %rsp
	movq	%rdx, -16(%rax)
	movq	%rsp, %rax
	leaq	-16(%rax), %rsi
	movq	%rsi, %rsp
	movq	$3, -16(%rax)
	.p2align	4, 0x90
.LBB1_7:                                # %for_condition
                                        # =>This Inner Loop Header: Depth=1
	movq	(%rsi), %rax
	cmpq	(%rcx), %rax
	jg	.LBB1_4
# %bb.8:                                # %for_body
                                        #   in Loop: Header=BB1_7 Depth=1
	movq	-8(%rbp), %rax
	cqto
	idivq	(%rsi)
	testq	%rdx, %rdx
	je	.LBB1_1
# %bb.9:                                # %merge17
                                        #   in Loop: Header=BB1_7 Depth=1
	incq	(%rsi)
	jmp	.LBB1_7
.Lfunc_end1:
	.size	ehPrimo, .Lfunc_end1-ehPrimo
	.cfi_endproc
                                        # -- End function
	.globl	contarPrimos                    # -- Begin function contarPrimos
	.p2align	4, 0x90
	.type	contarPrimos,@function
contarPrimos:                           # @contarPrimos
	.cfi_startproc
# %bb.0:                                # %entry
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, 16(%rsp)
	movq	$0, 8(%rsp)
	movq	$2, (%rsp)
	jmp	.LBB2_1
	.p2align	4, 0x90
.LBB2_4:                                # %merge
                                        #   in Loop: Header=BB2_1 Depth=1
	incq	(%rsp)
.LBB2_1:                                # %for_condition
                                        # =>This Inner Loop Header: Depth=1
	movq	(%rsp), %rax
	cmpq	16(%rsp), %rax
	jg	.LBB2_5
# %bb.2:                                # %for_body
                                        #   in Loop: Header=BB2_1 Depth=1
	movq	(%rsp), %rdi
	callq	ehPrimo@PLT
	testq	%rax, %rax
	je	.LBB2_4
# %bb.3:                                # %then
                                        #   in Loop: Header=BB2_1 Depth=1
	incq	8(%rsp)
	jmp	.LBB2_4
.LBB2_5:                                # %after_for
	movq	8(%rsp), %rax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end2:
	.size	contarPrimos, .Lfunc_end2-contarPrimos
	.cfi_endproc
                                        # -- End function
	.type	string_literal,@object          # @string_literal
	.data
	.globl	string_literal
string_literal:
	.asciz	"RESULT:"
	.size	string_literal, 8

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
