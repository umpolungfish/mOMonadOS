static char path[] = "/home/mrnob0dy666/imsgct/mOMonadOS/measurements/filetest_input.txt";
static char buf[64];

void _start() {
    __asm__ volatile (
        // fd = openat(AT_FDCWD, path, O_RDONLY, 0)
        "mov $257, %%rax\n"
        "mov $-100, %%rdi\n"
        "mov %0, %%rsi\n"
        "xor %%rdx, %%rdx\n"
        "xor %%r10, %%r10\n"
        "syscall\n"
        "mov %%rax, %%rbx\n"           // rbx = fd
        // read(fd, buf, 64)
        "mov $0, %%rax\n"
        "mov %%rbx, %%rdi\n"
        "mov %1, %%rsi\n"
        "mov $64, %%rdx\n"
        "syscall\n"
        "mov %%rax, %%rdx\n"           // rdx = bytes read
        // close(fd)
        "mov $3, %%rax\n"
        "mov %%rbx, %%rdi\n"
        "syscall\n"
        // write(1, buf, bytes read)
        "mov $1, %%rax\n"
        "mov $1, %%rdi\n"
        "mov %1, %%rsi\n"
        "syscall\n"
        // exit(11)
        "mov $60, %%rax\n"
        "mov $11, %%rdi\n"
        "syscall\n"
        :
        : "r"(path), "r"(buf)
        : "rax","rbx","rdi","rsi","rdx","r10"
    );
}
