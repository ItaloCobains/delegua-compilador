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
	movq	$3, (%rsp)
	movq	format_int@GOTPCREL(%rip), %rdi
	movl	$3, %esi
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
	.type	format_int,@object              # @format_int
	.data
	.globl	format_int
format_int:
	.asciz	"%lld\n"
	.size	format_int, 6

	.section	".note.GNU-stack","",@progbits
