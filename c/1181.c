#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_WORD_CNT (20000)
#define MAX_WORD_LEN (50)

int compare(const void *a, const void *b) {
    char *x = (char *)a;
    char *y = (char *)b;
    size_t len_x = strlen(x);
    size_t len_y = strlen(y);

    if (len_x == len_y) {
        return strcmp(x, y);
    }

    return len_x - len_y;
}

int main(void) {
    char words[MAX_WORD_CNT][MAX_WORD_LEN + 1];
    size_t word_cnt;
    size_t i = 0;

    scanf("%ld", &word_cnt);

    while (scanf("%s", words[i]) == 1) {
        i++;
    }

    qsort(words, word_cnt, sizeof(words[0]), compare);

    for (i = 0; i < word_cnt;) {
        puts(words[i]);
        size_t j = i + 1;

        for (; j < word_cnt; j++) {
            if (strcmp(words[i], words[j])) {
                break;
            }
        }

        i = j;
    }

    return 0;
}
