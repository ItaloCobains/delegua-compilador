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
	movq	$180, (%rsp)
	xorl	%eax, %eax
	testb	%al, %al
	jne	.LBB0_4
# %bb.1:                                # %then
	movq	format_str@GOTPCREL(%rip), %rdi
	movq	string_literal@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	cmpq	$79, (%rsp)
	jle	.LBB0_4
# %bb.2:                                # %then4
	movq	format_str.2@GOTPCREL(%rip), %rdi
	movq	string_literal.1@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	cmpq	$70, (%rsp)
	jl	.LBB0_4
# %bb.3:                                # %then9
	movq	format_str.4@GOTPCREL(%rip), %rdi
	movq	string_literal.3@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
.LBB0_4:                                # %merge
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.type	string_literal,@object          # @string_literal
	.data
	.globl	string_literal
	.p2align	4, 0x0
string_literal:
	.asciz	"Excelente! Nota A"
	.size	string_literal, 18

	.type	format_str,@object              # @format_str
	.globl	format_str
format_str:
	.asciz	"%s\n"
	.size	format_str, 4

	.type	string_literal.1,@object        # @string_literal.1
	.globl	string_literal.1
string_literal.1:
	.asciz	"a"
	.size	string_literal.1, 2

	.type	format_str.2,@object            # @format_str.2
	.globl	format_str.2
format_str.2:
	.asciz	"%s\n"
	.size	format_str.2, 4

	.type	string_literal.3,@object        # @string_literal.3
	.globl	string_literal.3
string_literal.3:
	.asciz	"s"
	.size	string_literal.3, 2

	.type	format_str.4,@object            # @format_str.4
	.globl	format_str.4
format_str.4:
	.asciz	"%s\n"
	.size	format_str.4, 4

	.section	".note.GNU-stack","",@progbits
