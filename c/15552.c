#include <stdio.h>

#define MAX_LEN (8 + (10 * 1000000) + 1)

int main(void) {
    char buf[MAX_LEN];
    size_t cnt = fread(buf, 1, MAX_LEN, stdin);

    buf[cnt] = '\0';
    size_t i = 0;

    while (buf[i] != '\n') {
        i++;
    }

    i++;
    char *pos = buf;

    while (i < cnt) {
        int a = 0;
        int b = 0;

        for (; buf[i] != ' '; i++) {
            a = a * 10 + (buf[i] - '0');
        }

        i++;

        for (; buf[i] != '\n'; i++) {
            b = b * 10 + (buf[i] - '0');
        }

        while (buf[i] == '\n') {
            i++;
        }

        int sum = a + b;
        int pow = 1;

        while (pow <= sum / 10) {
            pow *= 10;
        }

        for (; pow > 0; pow /= 10) {
            int digit = sum / pow;

            sum -= digit * pow;
            *pos = digit + '0';
            pos++;
        }

        *pos = '\n';
        pos++;
    }

    fwrite(buf, 1, pos - buf, stdout);

    return 0;
}
