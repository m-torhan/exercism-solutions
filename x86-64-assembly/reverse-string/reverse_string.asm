section .text
global reverse
reverse:
    mov    rax, -1
    xor    rcx,  rcx

.get_len:
    inc    rax
    cmp    byte [rdi + rax], 0
    jne    .get_len

    lea    rdx, [rax - 1]

    mov    bl, 2
    div    bl

.rev:
    cmp    cl, al
    je     .end

    mov    bl, [rdi + rcx]
    mov    bh, [rdi + rdx]
    mov    [rdi + rcx], bh
    mov    [rdi + rdx], bl

    inc    rcx
    dec    rdx

    jmp .rev

.end:
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
