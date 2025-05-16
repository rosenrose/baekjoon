#include <stdio.h>
#include <limits.h>

#define MOD (650011)

void add(int arr[], int val) {
    int i = val % MOD;
    
    if (i < 0) {
        i += MOD;
    }

    while (arr[i] != INT_MAX) {
        if (arr[i] == val) {
            return;
        }
        
        i = (i + 1) % MOD;
    }

    arr[i] = val;
}

int has(int arr[], int val) {
    int i = val % MOD;

    if (i < 0) {
        i += MOD;
    }

    while (arr[i] != INT_MAX) {
        if (arr[i] == val) {
            return 1;
        }
        
        i = (i + 1) % MOD;
    }

    return 0;
}

int main(void) {
    int arr[MOD];
    size_t i;

    for (i = 0; i < MOD; i++) {
        arr[i] = INT_MAX;
    }

    size_t n;
    int num;
    
    scanf("%ld", &n);
    
    for (i = 0; i < n; i++) {
        scanf("%d", &num);
        add(arr, num);
    }

    scanf("%ld", &n);
    
    while (scanf("%d", &num) == 1) {
        printf("%d ", has(arr, num));
    }

    return 0;
}
