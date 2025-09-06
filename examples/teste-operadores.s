	.text
	.file	"program"
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	$5, 16(%rsp)
	movq	$10, 8(%rsp)
	movq	format_str@GOTPCREL(%rip), %rdi
	movq	string_literal@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	16(%rsp), %rax
	cmpq	8(%rsp), %rax
	jge	.LBB0_2
# %bb.1:                                # %then
	movq	format_str.2@GOTPCREL(%rip), %rdi
	movq	string_literal.1@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
.LBB0_2:                                # %merge
	cmpq	$5, 16(%rsp)
	jg	.LBB0_4
# %bb.3:                                # %then5
	movq	format_str.4@GOTPCREL(%rip), %rdi
	movq	string_literal.3@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
.LBB0_4:                                # %merge6
	movq	8(%rsp), %rax
	cmpq	16(%rsp), %rax
	jg	.LBB0_5
# %bb.6:                                # %merge11
	cmpq	$10, 8(%rsp)
	jge	.LBB0_7
.LBB0_8:                                # %merge15
	cmpq	$5, 16(%rsp)
	je	.LBB0_9
.LBB0_10:                               # %merge19
	cmpq	$5, 8(%rsp)
	je	.LBB0_12
.LBB0_11:                               # %then22
	movq	format_str.12@GOTPCREL(%rip), %rdi
	movq	string_literal.11@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
.LBB0_12:                               # %merge23
	xorl	%eax, %eax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB0_5:                                # %then10
	.cfi_def_cfa_offset 32
	movq	format_str.6@GOTPCREL(%rip), %rdi
	movq	string_literal.5@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	cmpq	$10, 8(%rsp)
	jl	.LBB0_8
.LBB0_7:                                # %then14
	movq	format_str.8@GOTPCREL(%rip), %rdi
	movq	string_literal.7@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	cmpq	$5, 16(%rsp)
	jne	.LBB0_10
.LBB0_9:                                # %then18
	movq	format_str.10@GOTPCREL(%rip), %rdi
	movq	string_literal.9@GOTPCREL(%rip), %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	cmpq	$5, 8(%rsp)
	jne	.LBB0_11
	jmp	.LBB0_12
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.type	string_literal,@object          # @string_literal
	.data
	.globl	string_literal
	.p2align	4, 0x0
string_literal:
	.asciz	"Testando operadores de compara\303\247\303\243o:"
	.size	string_literal, 37

	.type	format_str,@object              # @format_str
	.globl	format_str
format_str:
	.asciz	"%s\n"
	.size	format_str, 4

	.type	string_literal.1,@object        # @string_literal.1
	.globl	string_literal.1
	.p2align	4, 0x0
string_literal.1:
	.asciz	"a < b \303\251 verdadeiro"
	.size	string_literal.1, 20

	.type	format_str.2,@object            # @format_str.2
	.globl	format_str.2
format_str.2:
	.asciz	"%s\n"
	.size	format_str.2, 4

	.type	string_literal.3,@object        # @string_literal.3
	.globl	string_literal.3
	.p2align	4, 0x0
string_literal.3:
	.asciz	"a <= 5 \303\251 verdadeiro"
	.size	string_literal.3, 21

	.type	format_str.4,@object            # @format_str.4
	.globl	format_str.4
format_str.4:
	.asciz	"%s\n"
	.size	format_str.4, 4

	.type	string_literal.5,@object        # @string_literal.5
	.globl	string_literal.5
	.p2align	4, 0x0
string_literal.5:
	.asciz	"b > a \303\251 verdadeiro"
	.size	string_literal.5, 20

	.type	format_str.6,@object            # @format_str.6
	.globl	format_str.6
format_str.6:
	.asciz	"%s\n"
	.size	format_str.6, 4

	.type	string_literal.7,@object        # @string_literal.7
	.globl	string_literal.7
	.p2align	4, 0x0
string_literal.7:
	.asciz	"b >= 10 \303\251 verdadeiro"
	.size	string_literal.7, 22

	.type	format_str.8,@object            # @format_str.8
	.globl	format_str.8
format_str.8:
	.asciz	"%s\n"
	.size	format_str.8, 4

	.type	string_literal.9,@object        # @string_literal.9
	.globl	string_literal.9
	.p2align	4, 0x0
string_literal.9:
	.asciz	"a == 5 \303\251 verdadeiro"
	.size	string_literal.9, 21

	.type	format_str.10,@object           # @format_str.10
	.globl	format_str.10
format_str.10:
	.asciz	"%s\n"
	.size	format_str.10, 4

	.type	string_literal.11,@object       # @string_literal.11
	.globl	string_literal.11
	.p2align	4, 0x0
string_literal.11:
	.asciz	"b != 5 \303\251 verdadeiro"
	.size	string_literal.11, 21

	.type	format_str.12,@object           # @format_str.12
	.globl	format_str.12
format_str.12:
	.asciz	"%s\n"
	.size	format_str.12, 4

	.section	".note.GNU-stack","",@progbits
