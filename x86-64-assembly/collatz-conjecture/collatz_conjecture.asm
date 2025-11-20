section .text
global steps
steps:
    mov    eax, edi
    xor    ebx, ebx

    cmp    eax, 0
    jle    .error
    test   eax, eax
    js     .error

.loop:
    cmp    eax, 1
    je     .end
    inc    ebx
    mov    ecx, eax
    and    ecx, 1
    cmp    ecx, 0
    jne    .odd

.even:
    shr    eax, 1
    jmp    .loop

.odd:
    lea    eax, [3*eax + 1]
    jmp    .loop

.error:
    mov    ebx, -1
    jmp    .end

.end:
    mov    eax, ebx
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
