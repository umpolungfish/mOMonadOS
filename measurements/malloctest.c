#include <stdlib.h>
#include <unistd.h>
int main() {
    char *p = malloc(64);
    for (int i = 0; i < 5; i++) p[i] = 'A' + i;
    write(1, p, 5);
    write(1, "\n", 1);
    return 3;
}
