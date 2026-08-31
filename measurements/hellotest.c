void _start() {
    const char msg[] = "hello from inside the twelve\n";
    __asm__ volatile (
        "mov $1, %%rax\n"
        "mov $1, %%rdi\n"
        "mov %0, %%rsi\n"
        "mov $30, %%rdx\n"
        "syscall\n"
        "mov $60, %%rax\n"
        "mov $7, %%rdi\n"
        "syscall\n"
        :
        : "r"(msg)
        : "rax","rdi","rsi","rdx"
    );
}
