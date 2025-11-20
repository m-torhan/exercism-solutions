section .text
global leap_year
leap_year:
    mov    eax, edi
    ror    rdi, 32
    mov    edx, edi
    rol    rdi, 32

    mov    ecx, 4
    div    ecx

    cmp    edx, 0
    jne    .ret_false

    mov    eax, edi
    ror    rdi, 32
    mov    edx, edi
    rol    rdi, 32

    mov    ecx, 400
    div    ecx

    cmp    edx, 0
    je     .ret_true

    mov    eax, edi
    ror    rdi, 32
    mov    edx, edi
    rol    rdi, 32

    mov    ecx, 100
    div    ecx

    cmp    edx, 0
    je     .ret_false

.ret_true:
    mov    rax, 1
    jmp    .end

.ret_false:
    xor    rax, rax

.end:
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
