#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <unistd.h>

int main() {
    pid_t pid = fork(); // 创建新进程

    if (pid < 0) {
        // fork失败
        fprintf(stderr, "Fork failed\n");
        return 1;
    }

    printf("start\n");

    if (pid == 0) {
        // 子进程
        execlp("./DDSHelloWorldExample", "DDSHelloWorldExample", "publisher", NULL); // 执行ls命令
    } else {
        // 父进程
        execlp("./DDSHelloWorldExample", "DDSHelloWorldExample", "subscriber", NULL); // 执行ls命令
    }

    return 0;
}
