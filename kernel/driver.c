#include <stdio.h>

void kernel_driver_hello() {
    printf("Kernel driver: Hello from C!\n");
    fflush(stdout);
}

int kernel_driver_add(int a, int b) {
    printf("Kernel driver: adding %d + %d\n", a, b);
    fflush(stdout);
    return a + b;
}

