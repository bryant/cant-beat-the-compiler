# x64 assembly hand-written sort routine (explicit stack management)
    .intel_syntax noprefix

    .globl sortRoutine
sortRoutine:
    # rcx = items
    # rdx = count
    mov rcx, rdi
    mov rdx, rsi
    dec rdx
    jz done

    # push a sentinel
    xor rax, rax
    push rax
    push rax

donext:
    # Pick the pivot.
    mov r8, [rcx+rdx*8]    # r8 = pivot data
    xor r9, r9            # r9 = low
    xor r10, r10        # r10 = pos
partition:
    cmp [rcx+r10*8], r8d
    jg noswap

    # swap elements
    mov rax, [rcx+r10*8]
    mov r11, [rcx+r9*8]
    mov [rcx+r9*8], rax
    mov [rcx+r10*8], r11
    inc r9

noswap:
    inc r10
    cmp r10, rdx
    jb partition

    # move pivot into place
    mov rax, [rcx+r9*8]
    mov [rcx+rdx*8], rax
    mov [rcx+r9*8], r8
    
    # recurse
    sub rdx, r9
    sub rdx, 1
    jbe nopush

    # push right side
    lea rax, [rcx+r9*8+8]
    push rax
    push rdx
nopush:

    # move to left side
    mov rdx, r9
    sub rdx, 1
    ja donext

    # no left side, pop off stack
    pop rdx
    pop rcx
    test rcx, rcx
    jnz donext

done:
    ret
