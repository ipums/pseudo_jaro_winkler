	.text
	.file	"rlink.b6bbdb73-cgu.0"
	.section	.text._ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h6cfc6624dec9b053E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h6cfc6624dec9b053E,@function
_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h6cfc6624dec9b053E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	*%rdi
	movq	%rsp, %rax
	#APP
	#NO_APP
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h6cfc6624dec9b053E, .Lfunc_end0-_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h6cfc6624dec9b053E
	.cfi_endproc

	.section	.text._ZN3std2rt10lang_start17hbdbc343fcf75a6a1E,"ax",@progbits
	.hidden	_ZN3std2rt10lang_start17hbdbc343fcf75a6a1E
	.globl	_ZN3std2rt10lang_start17hbdbc343fcf75a6a1E
	.p2align	4, 0x90
	.type	_ZN3std2rt10lang_start17hbdbc343fcf75a6a1E,@function
_ZN3std2rt10lang_start17hbdbc343fcf75a6a1E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, (%rsp)
	leaq	.L__unnamed_1(%rip), %rsi
	movq	%rsp, %rdi
	callq	*_ZN3std2rt19lang_start_internal17h7e2cee8c90d4a4d3E@GOTPCREL(%rip)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	_ZN3std2rt10lang_start17hbdbc343fcf75a6a1E, .Lfunc_end1-_ZN3std2rt10lang_start17hbdbc343fcf75a6a1E
	.cfi_endproc

	.section	".text._ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h99fb92bf24698b42E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h99fb92bf24698b42E,@function
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h99fb92bf24698b42E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h6cfc6624dec9b053E
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end2:
	.size	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h99fb92bf24698b42E, .Lfunc_end2-_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h99fb92bf24698b42E
	.cfi_endproc

	.section	".text._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h112bb2f205c91acaE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h112bb2f205c91acaE,@function
_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h112bb2f205c91acaE:
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movq	%rsi, %rbx
	movq	(%rdi), %r14
	movq	%rsi, %rdi
	callq	*_ZN4core3fmt9Formatter15debug_lower_hex17h186fc2d370d14809E@GOTPCREL(%rip)
	testb	%al, %al
	je	.LBB3_1
	movq	%r14, %rdi
	movq	%rbx, %rsi
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	jmpq	*_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u64$GT$3fmt17h62ed73e546de8a3fE@GOTPCREL(%rip)
.LBB3_1:
	.cfi_def_cfa_offset 32
	movq	%rbx, %rdi
	callq	*_ZN4core3fmt9Formatter15debug_upper_hex17hb4aa2072a38f4390E@GOTPCREL(%rip)
	movq	%r14, %rdi
	movq	%rbx, %rsi
	addq	$8, %rsp
	testb	%al, %al
	je	.LBB3_4
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	jmpq	*_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u64$GT$3fmt17h144f533d90bc2e0bE@GOTPCREL(%rip)
.LBB3_4:
	.cfi_def_cfa_offset 32
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	jmpq	*_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17hf962877c8c9b076cE@GOTPCREL(%rip)
.Lfunc_end3:
	.size	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h112bb2f205c91acaE, .Lfunc_end3-_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h112bb2f205c91acaE
	.cfi_endproc

	.section	".text._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hfc9aff6d2e08f530E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hfc9aff6d2e08f530E,@function
_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hfc9aff6d2e08f530E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$24, %rsp
	.cfi_def_cfa_offset 80
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	(%rdi), %rbx
	movq	%rsi, %rdi
	callq	*_ZN4core3fmt9Formatter10debug_list17h72d8cd828e9e459eE@GOTPCREL(%rip)
	movq	%rdx, 16(%rsp)
	movq	%rax, 8(%rsp)
	leaq	8(%rbx), %rbp
	movq	%rbx, (%rsp)
	leaq	.L__unnamed_2(%rip), %rdx
	movq	_ZN4core3fmt8builders9DebugList5entry17hc9ded61ffda669f4E@GOTPCREL(%rip), %r14
	leaq	8(%rsp), %r12
	movq	%rsp, %r15
	movq	%r12, %rdi
	movq	%r15, %rsi
	callq	*%r14
	leaq	16(%rbx), %r13
	movq	%rbp, (%rsp)
	movq	%r12, %rdi
	movq	%r15, %rsi
	leaq	.L__unnamed_2(%rip), %rdx
	callq	*%r14
	leaq	24(%rbx), %rbp
	movq	%r13, (%rsp)
	movq	%r12, %rdi
	movq	%r15, %rsi
	leaq	.L__unnamed_2(%rip), %rdx
	callq	*%r14
	leaq	32(%rbx), %r13
	movq	%rbp, (%rsp)
	movq	%r12, %rdi
	movq	%r15, %rsi
	leaq	.L__unnamed_2(%rip), %rdx
	callq	*%r14
	leaq	40(%rbx), %rbp
	movq	%r13, (%rsp)
	movq	%r12, %rdi
	movq	%r15, %rsi
	leaq	.L__unnamed_2(%rip), %rdx
	callq	*%r14
	leaq	48(%rbx), %r13
	movq	%rbp, (%rsp)
	movq	%r12, %rdi
	movq	%r15, %rsi
	leaq	.L__unnamed_2(%rip), %rdx
	movq	%rdx, %rbp
	callq	*%r14
	addq	$56, %rbx
	movq	%r13, (%rsp)
	movq	%r12, %rdi
	movq	%r15, %rsi
	movq	%rbp, %rdx
	callq	*%r14
	movq	%rbx, (%rsp)
	movq	%r12, %rdi
	movq	%r15, %rsi
	movq	%rbp, %rdx
	callq	*%r14
	movq	%r12, %rdi
	callq	*_ZN4core3fmt8builders9DebugList6finish17h81ca94db0160ef80E@GOTPCREL(%rip)
	addq	$24, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end4:
	.size	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hfc9aff6d2e08f530E, .Lfunc_end4-_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hfc9aff6d2e08f530E
	.cfi_endproc

	.section	".text._ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hffab3d40480fd445E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hffab3d40480fd445E,@function
_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hffab3d40480fd445E:
	.cfi_startproc
	movq	%rsi, %rdx
	movq	(%rdi), %rax
	movq	8(%rdi), %rsi
	movq	%rax, %rdi
	jmpq	*_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h646e44b102e27d7dE@GOTPCREL(%rip)
.Lfunc_end5:
	.size	_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hffab3d40480fd445E, .Lfunc_end5-_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hffab3d40480fd445E
	.cfi_endproc

	.section	".text._ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he3fc195741715dd2E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he3fc195741715dd2E,@function
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he3fc195741715dd2E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h6cfc6624dec9b053E
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end6:
	.size	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he3fc195741715dd2E, .Lfunc_end6-_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he3fc195741715dd2E
	.cfi_endproc

	.section	".text._ZN4core3ptr28drop_in_place$LT$$RF$u64$GT$17h5f63b0be860ef0dcE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr28drop_in_place$LT$$RF$u64$GT$17h5f63b0be860ef0dcE,@function
_ZN4core3ptr28drop_in_place$LT$$RF$u64$GT$17h5f63b0be860ef0dcE:
	.cfi_startproc
	retq
.Lfunc_end7:
	.size	_ZN4core3ptr28drop_in_place$LT$$RF$u64$GT$17h5f63b0be860ef0dcE, .Lfunc_end7-_ZN4core3ptr28drop_in_place$LT$$RF$u64$GT$17h5f63b0be860ef0dcE
	.cfi_endproc

	.section	.text._ZN5rlink4main17h6d5280a8f796da07E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5rlink4main17h6d5280a8f796da07E,@function
_ZN5rlink4main17h6d5280a8f796da07E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$200, %rsp
	.cfi_def_cfa_offset 256
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	callq	*_ZN3std4time7Instant3now17hb703a768d13861d2E@GOTPCREL(%rip)
	movq	%rax, 72(%rsp)
	movq	%rdx, 80(%rsp)
	xorl	%r10d, %r10d
	movl	$2, %ecx
	movl	$3, %edx
	movl	$4, %esi
	movl	$5, %r13d
	movl	$1, %r14d
	xorl	%r15d, %r15d
	xorl	%r12d, %r12d
	xorl	%r11d, %r11d
	jmp	.LBB8_1
	.p2align	4, 0x90
.LBB8_38:
	movq	%r13, %rsi
.LBB8_41:
	imulq	%rsi, %rax
	movq	%rax, %r13
.LBB8_42:
	addl	$1, %r11d
	movq	%rbx, %rsi
	movq	%r8, %rcx
	movq	%r9, %r15
	cmpl	$1000000, %r11d
	je	.LBB8_43
.LBB8_1:
	movl	$1, %edi
	testl	%r12d, %r12d
	je	.LBB8_6
	cmpl	$1, %r12d
	jne	.LBB8_4
	movl	$1, %edi
	jmp	.LBB8_5
	.p2align	4, 0x90
.LBB8_4:
	movq	%r12, %rax
	imulq	%r12, %rax
	testb	$1, %r12b
	cmoveq	%r14, %r12
	movq	%r12, %rdi
	movq	%rax, %r12
.LBB8_5:
	imulq	%r12, %rdi
.LBB8_6:
	movq	%rdi, %r12
	movl	$1, %eax
	movl	$1, %r9d
	testl	%r15d, %r15d
	je	.LBB8_11
	cmpl	$1, %r15d
	jne	.LBB8_9
	movl	$1, %r9d
	jmp	.LBB8_10
	.p2align	4, 0x90
.LBB8_9:
	movq	%r15, %rdi
	imulq	%r15, %rdi
	testb	$1, %r15b
	cmoveq	%r14, %r15
	movq	%r15, %r9
	movq	%rdi, %r15
.LBB8_10:
	imulq	%r15, %r9
.LBB8_11:
	testl	%r10d, %r10d
	je	.LBB8_12
	cmpl	$1, %r10d
	je	.LBB8_15
	movq	%r10, %rdi
	imulq	%r10, %rdi
	testb	$1, %r10b
	cmoveq	%r14, %r10
	movq	%r10, %rax
	movq	%rdi, %r10
.LBB8_15:
	imulq	%rax, %r10
	movl	$1, %ebx
	movl	$1, %r8d
	testl	%ecx, %ecx
	jne	.LBB8_17
.LBB8_22:
	testl	%edx, %edx
	je	.LBB8_23
.LBB8_24:
	cmpl	$1, %edx
	jne	.LBB8_26
	movq	%rdx, %rax
	jmp	.LBB8_28
	.p2align	4, 0x90
.LBB8_12:
	movl	$1, %r10d
	movl	$1, %ebx
	movl	$1, %r8d
	testl	%ecx, %ecx
	je	.LBB8_22
.LBB8_17:
	movl	$1, %r8d
	cmpl	$1, %ecx
	jne	.LBB8_19
	movq	%rcx, %rdi
	jmp	.LBB8_21
	.p2align	4, 0x90
.LBB8_26:
	movl	$1, %ebx
	movq	%rdx, %rax
	movl	%edx, %edi
	.p2align	4, 0x90
.LBB8_27:
	testb	$1, %dl
	movq	%rax, %rcx
	cmoveq	%r14, %rcx
	imulq	%rcx, %rbx
	shrl	%edi
	imulq	%rax, %rax
	cmpl	$3, %edx
	movl	%edi, %edx
	ja	.LBB8_27
.LBB8_28:
	imulq	%rax, %rbx
	movq	%rbx, %rdx
	movl	$1, %eax
	movl	$1, %ebx
	testl	%esi, %esi
	jne	.LBB8_30
.LBB8_35:
	testl	%r13d, %r13d
	je	.LBB8_36
.LBB8_37:
	cmpl	$1, %r13d
	je	.LBB8_38
	movl	$1, %eax
	movq	%r13, %rsi
	movl	%r13d, %edi
	.p2align	4, 0x90
.LBB8_40:
	testb	$1, %r13b
	movq	%rsi, %rcx
	cmoveq	%r14, %rcx
	imulq	%rcx, %rax
	shrl	%edi
	imulq	%rsi, %rsi
	cmpl	$3, %r13d
	movl	%edi, %r13d
	ja	.LBB8_40
	jmp	.LBB8_41
	.p2align	4, 0x90
.LBB8_19:
	movq	%rcx, %rdi
	movl	%ecx, %ebp
	.p2align	4, 0x90
.LBB8_20:
	testb	$1, %cl
	movq	%rdi, %rax
	cmoveq	%r14, %rax
	imulq	%rax, %r8
	shrl	%ebp
	imulq	%rdi, %rdi
	cmpl	$3, %ecx
	movl	%ebp, %ecx
	ja	.LBB8_20
.LBB8_21:
	imulq	%rdi, %r8
	testl	%edx, %edx
	jne	.LBB8_24
.LBB8_23:
	movl	$1, %edx
	movl	$1, %eax
	movl	$1, %ebx
	testl	%esi, %esi
	je	.LBB8_35
.LBB8_30:
	movl	$1, %ebx
	cmpl	$1, %esi
	jne	.LBB8_32
	movq	%rsi, %rdi
	jmp	.LBB8_34
	.p2align	4, 0x90
.LBB8_32:
	movq	%rsi, %rdi
	movl	%esi, %ebp
	.p2align	4, 0x90
.LBB8_33:
	testb	$1, %sil
	movq	%rdi, %rcx
	cmoveq	%r14, %rcx
	imulq	%rcx, %rbx
	shrl	%ebp
	imulq	%rdi, %rdi
	cmpl	$3, %esi
	movl	%ebp, %esi
	ja	.LBB8_33
.LBB8_34:
	imulq	%rdi, %rbx
	testl	%r13d, %r13d
	jne	.LBB8_37
.LBB8_36:
	movl	$1, %r13d
	jmp	.LBB8_42
.LBB8_43:
	movq	%r12, 136(%rsp)
	movq	%r9, 144(%rsp)
	movq	%r10, 152(%rsp)
	movq	$1, 160(%rsp)
	movq	%r8, 168(%rsp)
	movq	%rdx, 176(%rsp)
	movq	%rbx, 184(%rsp)
	movq	%r13, 192(%rsp)
	leaq	136(%rsp), %r14
	movq	%r14, (%rsp)
	leaq	.L__unnamed_3(%rip), %rax
	movq	%rax, 8(%rsp)
	leaq	_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hffab3d40480fd445E(%rip), %rax
	movq	%rax, 16(%rsp)
	leaq	.L__unnamed_4(%rip), %rcx
	movq	%rcx, 24(%rsp)
	movq	_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hc9fcdb231e27a5cbE@GOTPCREL(%rip), %rcx
	movq	%rcx, 32(%rsp)
	leaq	.L__unnamed_5(%rip), %rcx
	movq	%rcx, 40(%rsp)
	movq	%rax, 48(%rsp)
	movq	%rsp, %rax
	movq	%rax, 56(%rsp)
	leaq	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hfc9aff6d2e08f530E(%rip), %rax
	movq	%rax, 64(%rsp)
	leaq	.L__unnamed_6(%rip), %rax
	movq	%rax, 88(%rsp)
	movq	$5, 96(%rsp)
	leaq	.L__unnamed_7(%rip), %rax
	movq	%rax, 104(%rsp)
	movq	$4, 112(%rsp)
	leaq	8(%rsp), %rax
	movq	%rax, 120(%rsp)
	movq	$4, 128(%rsp)
	leaq	88(%rsp), %rbx
	movq	%rbx, %rdi
	callq	*_ZN3std2io5stdio7_eprint17hd3f3a94cd4dfda51E@GOTPCREL(%rip)
	leaq	72(%rsp), %rdi
	callq	*_ZN3std4time7Instant7elapsed17ha9954b30ffc6b968E@GOTPCREL(%rip)
	movl	%edx, %ecx
	movl	$1000, %edx
	mulq	%rdx
	movl	%ecx, %ecx
	imulq	$1125899907, %rcx, %rcx
	shrq	$50, %rcx
	addq	%rax, %rcx
	adcq	$0, %rdx
	movq	%rcx, 136(%rsp)
	movq	%rdx, 144(%rsp)
	movq	%r14, 88(%rsp)
	movq	_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..Display$u20$for$u20$u128$GT$3fmt17h197edd890809efd2E@GOTPCREL(%rip), %rax
	movq	%rax, 96(%rsp)
	leaq	.L__unnamed_8(%rip), %rax
	movq	%rax, 8(%rsp)
	movq	$2, 16(%rsp)
	movq	$0, 24(%rsp)
	movq	%rbx, 40(%rsp)
	movq	$1, 48(%rsp)
	leaq	8(%rsp), %rdi
	callq	*_ZN3std2io5stdio6_print17hf2e9dc80124c4394E@GOTPCREL(%rip)
	addq	$200, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end8:
	.size	_ZN5rlink4main17h6d5280a8f796da07E, .Lfunc_end8-_ZN5rlink4main17h6d5280a8f796da07E
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rsi, %rcx
	movslq	%edi, %rdx
	leaq	_ZN5rlink4main17h6d5280a8f796da07E(%rip), %rax
	movq	%rax, (%rsp)
	leaq	.L__unnamed_1(%rip), %rsi
	movq	%rsp, %rdi
	callq	*_ZN3std2rt19lang_start_internal17h7e2cee8c90d4a4d3E@GOTPCREL(%rip)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end9:
	.size	main, .Lfunc_end9-main
	.cfi_endproc

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	3
.L__unnamed_1:
	.quad	_ZN4core3ptr28drop_in_place$LT$$RF$u64$GT$17h5f63b0be860ef0dcE
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h99fb92bf24698b42E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h99fb92bf24698b42E
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he3fc195741715dd2E
	.size	.L__unnamed_1, 48

	.type	.L__unnamed_2,@object
	.section	.data.rel.ro..L__unnamed_2,"aw",@progbits
	.p2align	3
.L__unnamed_2:
	.quad	_ZN4core3ptr28drop_in_place$LT$$RF$u64$GT$17h5f63b0be860ef0dcE
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h112bb2f205c91acaE
	.size	.L__unnamed_2, 32

	.type	.L__unnamed_9,@object
	.section	.rodata..L__unnamed_9,"a",@progbits
.L__unnamed_9:
	.ascii	"src/main.rs"
	.size	.L__unnamed_9, 11

	.type	.L__unnamed_10,@object
	.section	.rodata..L__unnamed_10,"a",@progbits
.L__unnamed_10:
	.byte	91
	.size	.L__unnamed_10, 1

	.type	.L__unnamed_11,@object
	.section	.rodata..L__unnamed_11,"a",@progbits
.L__unnamed_11:
	.byte	58
	.size	.L__unnamed_11, 1

	.type	.L__unnamed_12,@object
	.section	.rodata..L__unnamed_12,"a",@progbits
.L__unnamed_12:
	.ascii	"] "
	.size	.L__unnamed_12, 2

	.type	.L__unnamed_13,@object
	.section	.rodata..L__unnamed_13,"a",@progbits
.L__unnamed_13:
	.ascii	" = "
	.size	.L__unnamed_13, 3

	.type	.L__unnamed_14,@object
	.section	.rodata..L__unnamed_14,"a",@progbits
.L__unnamed_14:
	.byte	10
	.size	.L__unnamed_14, 1

	.type	.L__unnamed_6,@object
	.section	.data.rel.ro..L__unnamed_6,"aw",@progbits
	.p2align	3
.L__unnamed_6:
	.quad	.L__unnamed_10
	.asciz	"\001\000\000\000\000\000\000"
	.quad	.L__unnamed_11
	.asciz	"\001\000\000\000\000\000\000"
	.quad	.L__unnamed_12
	.asciz	"\002\000\000\000\000\000\000"
	.quad	.L__unnamed_13
	.asciz	"\003\000\000\000\000\000\000"
	.quad	.L__unnamed_14
	.asciz	"\001\000\000\000\000\000\000"
	.size	.L__unnamed_6, 80

	.type	.L__unnamed_3,@object
	.section	.data.rel.ro..L__unnamed_3,"aw",@progbits
	.p2align	3
.L__unnamed_3:
	.quad	.L__unnamed_9
	.asciz	"\013\000\000\000\000\000\000"
	.size	.L__unnamed_3, 16

	.type	.L__unnamed_4,@object
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2
.L__unnamed_4:
	.asciz	"\n\000\000"
	.size	.L__unnamed_4, 4

	.type	.L__unnamed_15,@object
	.section	.rodata..L__unnamed_15,"a",@progbits
.L__unnamed_15:
	.ascii	"arr"
	.size	.L__unnamed_15, 3

	.type	.L__unnamed_5,@object
	.section	.data.rel.ro..L__unnamed_5,"aw",@progbits
	.p2align	3
.L__unnamed_5:
	.quad	.L__unnamed_15
	.asciz	"\003\000\000\000\000\000\000"
	.size	.L__unnamed_5, 16

	.type	.L__unnamed_7,@object
	.section	.rodata..L__unnamed_7,"a",@progbits
	.p2align	3
.L__unnamed_7:
	.asciz	"\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\000\000\000\000\003\000\000\000\000\000\000\000\001\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\000\000\000\000\003\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\000\000\000\000\003\000\000\000\000\000\000\000\003\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\004\000\000\000\003\000\000\000\000\000\000"
	.size	.L__unnamed_7, 224

	.type	.L__unnamed_16,@object
	.section	.rodata..L__unnamed_16,"a",@progbits
.L__unnamed_16:
	.size	.L__unnamed_16, 0

	.type	.L__unnamed_17,@object
	.section	.rodata.cst4,"aM",@progbits,4
.L__unnamed_17:
	.ascii	" ms\n"
	.size	.L__unnamed_17, 4

	.type	.L__unnamed_8,@object
	.section	.data.rel.ro..L__unnamed_8,"aw",@progbits
	.p2align	3
.L__unnamed_8:
	.quad	.L__unnamed_16
	.zero	8
	.quad	.L__unnamed_17
	.asciz	"\004\000\000\000\000\000\000"
	.size	.L__unnamed_8, 32

	.section	".note.GNU-stack","",@progbits
