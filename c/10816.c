#include <stdio.h>
#include <limits.h>

#define MOD (650011)

void add(int keys[], int values[], int input) {
    int i = input % MOD;
    
    if (i < 0) {
        i += MOD;
    }

    while (keys[i] != INT_MAX) {
        if (keys[i] == input) {
            break;
        }
        
        i = (i + 1) % MOD;
    }

    keys[i] = input;
    values[i]++;
}

int get(int keys[], int values[], int input) {
    int i = input % MOD;

    if (i < 0) {
        i += MOD;
    }

    while (keys[i] != INT_MAX) {
        if (keys[i] == input) {
            return values[i];
        }
        
        i = (i + 1) % MOD;
    }

    return 0;
}

int main(void) {
    int keys[MOD];
    int values[MOD] = { 0, };
    size_t i;

    for (i = 0; i < MOD; i++) {
        keys[i] = INT_MAX;
    }

    size_t n;
    int num;

    scanf("%ld", &n);
    
    for (i = 0; i < n; i++) {
        scanf("%d", &num);
        add(keys, values, num);
    }

    scanf("%ld", &n);
    
    while (scanf("%d", &num) == 1) {
        printf("%d ", get(keys, values, num));
    }

    return 0;
}
