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
	movq	$5, (%rsp)
	movb	$1, %al
	testb	%al, %al
	jne	.LBB0_3
# %bb.1:                                # %then
	movq	format_str@GOTPCREL(%rip), %rdi
	movq	string_literal@GOTPCREL(%rip), %rsi
	jmp	.LBB0_2
.LBB0_3:                                # %else
	movq	format_str.2@GOTPCREL(%rip), %rdi
	movq	string_literal.1@GOTPCREL(%rip), %rsi
.LBB0_2:                                # %merge
	xorl	%eax, %eax
	callq	printf@PLT
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
	.asciz	"x \303\251 maior que 10"
	.size	string_literal, 18

	.type	format_str,@object              # @format_str
	.globl	format_str
format_str:
	.asciz	"%s\n"
	.size	format_str, 4

	.type	string_literal.1,@object        # @string_literal.1
	.globl	string_literal.1
	.p2align	4, 0x0
string_literal.1:
	.asciz	"x \303\251 menor ou igual a 10"
	.size	string_literal.1, 25

	.type	format_str.2,@object            # @format_str.2
	.globl	format_str.2
format_str.2:
	.asciz	"%s\n"
	.size	format_str.2, 4

	.section	".note.GNU-stack","",@progbits
