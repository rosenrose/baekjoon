#include <stdio.h>

#define CNT (5000000)
#define BUF_LEN (8 + (9 * CNT))

int main(void) {
    char buf[BUF_LEN];
    long long sum = 0;

    fread(buf, 1, BUF_LEN, stdin);
    char* pos = buf;

    while (*pos != '\n') {
        pos++;
    }

    pos++;

    for (int i = 0; i < CNT; i++) {
        long long num = 0;

        for (; *pos != '\n'; pos++) {
            num = num * 10 + (*pos - '0');
        }

        sum += num;
        pos++;
    }

    printf("%d\n%lld", CNT, sum);

    return 0;
}
