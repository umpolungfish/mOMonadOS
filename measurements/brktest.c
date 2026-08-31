void _start() {
    __asm__ volatile (
        // brk(0) -> current break in rax
        "mov $12, %%rax\n"
        "mov $0, %%rdi\n"
        "syscall\n"
        "mov %%rax, %%rbx\n"          // rbx = old break
        "add $4096, %%rbx\n"
        // brk(old+4096) -> grow the heap
        "mov $12, %%rax\n"
        "mov %%rbx, %%rdi\n"
        "syscall\n"
        // write 3 bytes at the freshly-grown heap address into the heap itself
        "mov $12, %%rax\n"            // brk(0) again to get the (new) break
        "mov $0, %%rdi\n"
        "syscall\n"
        "sub $16, %%rax\n"            // a valid address just below the new break
        "movb $72, (%%rax)\n"         // 'H'
        "movb $73, 1(%%rax)\n"        // 'I'
        "movb $10, 2(%%rax)\n"        // '\n'
        "mov %%rax, %%rsi\n"
        // write(1, rsi, 3)
        "mov $1, %%rax\n"
        "mov $1, %%rdi\n"
        "mov $3, %%rdx\n"
        "syscall\n"
        // mmap(0, 4096, PROT_READ|WRITE, MAP_PRIVATE|ANON, -1, 0)
        "mov $9, %%rax\n"
        "xor %%rdi, %%rdi\n"
        "mov $4096, %%rsi\n"
        "mov $3, %%rdx\n"
        "mov $0x22, %%r10\n"
        "mov $-1, %%r8\n"
        "xor %%r9, %%r9\n"
        "syscall\n"
        "movb $77, (%%rax)\n"         // 'M'
        "movb $77, 1(%%rax)\n"        // 'M'
        "movb $10, 2(%%rax)\n"
        "mov %%rax, %%rsi\n"
        "mov $1, %%rax\n"
        "mov $1, %%rdi\n"
        "mov $3, %%rdx\n"
        "syscall\n"
        // exit(9)
        "mov $60, %%rax\n"
        "mov $9, %%rdi\n"
        "syscall\n"
        :
        :
        : "rax","rbx","rdi","rsi","rdx","r8","r9","r10"
    );
}
