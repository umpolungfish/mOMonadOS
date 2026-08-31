void _start() {
    __asm__ volatile (
        "mov $60, %rax\n"
        "mov $42, %rdi\n"
        "syscall\n"
    );
}
