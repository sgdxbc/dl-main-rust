#include <dlfcn.h>
#include <stdio.h>

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "argument error\n");
        return 1;
    }
    char *lib_path = argv[1];
    void *h = dlopen(lib_path, RTLD_LAZY);
    if (!h) {
        fprintf(stderr, "open error\n");
        return 1;
    }
    int (*f)(int, char **) = dlsym(h, "main");
    if (!f) {
        fprintf(stderr, "symbol error\n");
    }
    char *args[] = {
        "hello",
        "world",
        "42",
    };
    int result = f(3, args);
    if (result) {
        fprintf(stderr, "execution error\n");
    }

    return result;
}