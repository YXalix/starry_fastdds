#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <linux/netlink.h>
#include <linux/rtnetlink.h>
#include <linux/types.h>

#define MAX_PAYLOAD 4096

int main() {
    int sock_fd;
    struct sockaddr_nl src_addr, dest_addr;
    struct nlmsghdr *nlh;
    char buffer[MAX_PAYLOAD];

    // 创建一个套接字
    sock_fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE);
    if (sock_fd < 0) {
        perror("socket");
        exit(EXIT_FAILURE);
    }

    // memset(&src_addr, 0, sizeof(src_addr));
    // src_addr.nl_family = AF_NETLINK;
    // src_addr.nl_pid = getpid();
    // src_addr.nl_groups = 0;

    // if (bind(sock_fd, (struct sockaddr *)&src_addr, sizeof(src_addr)) < 0) {
    //     perror("bind");
    //     close(sock_fd);
    //     exit(EXIT_FAILURE);
    // }

    // 初始化目标地址
    memset(&dest_addr, 0, sizeof(dest_addr));
    dest_addr.nl_family = AF_NETLINK;
    dest_addr.nl_pid = 0;  // 对内核发送消息

    // 初始化消息头
    nlh = (struct nlmsghdr *)malloc(NLMSG_SPACE(MAX_PAYLOAD));
    memset(nlh, 0, NLMSG_SPACE(MAX_PAYLOAD));
    nlh->nlmsg_len = 20;
    nlh->nlmsg_type = RTM_GETLINK;
    nlh->nlmsg_pid = getpid();
    nlh->nlmsg_flags = NLM_F_REQUEST | NLM_F_DUMP;
    nlh->nlmsg_seq = 1;

    // 发送消息
    if (sendto(sock_fd, nlh, nlh->nlmsg_len, 0, (struct sockaddr *)&dest_addr, sizeof(dest_addr)) < 0) {
        perror("sendto");
        close(sock_fd);
        free(nlh);
        exit(EXIT_FAILURE);
    }

    // 接收消息，直到接收到长度为20的消息作为结束标志
    printf("Received GETLINK packets in u8 format:\n");
    int len = 0;
    while (1) {
        ssize_t recv_len = recvfrom(sock_fd, buffer, sizeof(buffer), 0, NULL, NULL);
        if (recv_len < 0) {
            perror("recvfrom");
            close(sock_fd);
            free(nlh);
            exit(EXIT_FAILURE);
        }
        
        // 打印接收到的包
        len += recv_len;
        printf("[");
        for (ssize_t i = 0; i < recv_len; ++i) {
            printf("0x%02x, ", (unsigned char)buffer[i]);
            // if ((i + 1) % 16 == 0)
            //     printf("\n");
        }
        printf("]\n");

        // 检查是否接收到长度为20的消息
        if (recv_len == 20)
            break;
    }

    printf("\nReceived GETLINK len %d\n", len);


    nlh->nlmsg_type = RTM_GETADDR;
    // 发送消息
    if (sendto(sock_fd, nlh, nlh->nlmsg_len, 0, (struct sockaddr *)&dest_addr, sizeof(dest_addr)) < 0) {
        perror("sendto");
        close(sock_fd);
        free(nlh);
        exit(EXIT_FAILURE);
    }

    // 接收消息，直到接收到长度为20的消息作为结束标志
    printf("Received GETADDR packets in u8 format:\n");
    len = 0;
    while (1) {
        ssize_t recv_len = recvfrom(sock_fd, buffer, sizeof(buffer), 0, NULL, NULL);
        if (recv_len < 0) {
            perror("recvfrom");
            close(sock_fd);
            free(nlh);
            exit(EXIT_FAILURE);
        }
        
        // 打印接收到的包
        len += recv_len;
        printf("[");
        for (ssize_t i = 0; i < recv_len; ++i) {
            printf("0x%02x, ", (unsigned char)buffer[i]);
            // if ((i + 1) % 16 == 0)
            //     printf("\n");
        }
        printf("]\n");

        // 检查是否接收到长度为20的消息
        if (recv_len == 20)
            break;
    }

    printf("\nReceived GETADDR len %d\n", len);


    // 释放资源
    close(sock_fd);
    free(nlh);

    return 0;
}