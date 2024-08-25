#include <stdio.h>

#define MAX_CNT (1000000)

void merge_sort(int arr[], size_t len, int tmp[]) {
    if (len <= 1) {
        return;
    }
    
    size_t half = len >> 1;
    
    merge_sort(arr, half, tmp);
    merge_sort(arr + half, len - half, tmp);
    
    size_t i;
    size_t left_idx = 0;
    size_t right_idx = half;
    
    for (i = 0; i < len; i++) {
        if (left_idx < half && right_idx < len) {
            if (arr[left_idx] < arr[right_idx]) {
                tmp[i] = arr[left_idx];
                left_idx++;
            } else {
                tmp[i] = arr[right_idx];
                right_idx++;
            }
        } else {
            if (left_idx < half) {
                tmp[i] = arr[left_idx];
                left_idx++;
            } else {
                tmp[i] = arr[right_idx];
                right_idx++;
            }
        }
    }
    
    for (i = 0; i < len; i++) {
        arr[i] = tmp[i];
    }
}

int main(void) {
    int nums[MAX_CNT];
    int tmp[MAX_CNT];
    size_t cnt;
    size_t i = 0;
    
    scanf("%ld", &cnt);
    
    while (scanf("%d", &nums[i]) == 1) {
        i++;
    }

    merge_sort(nums, cnt, tmp);
    
    for (i = 0; i < cnt; i++) {
        printf("%d\n", nums[i]);
    }
    
    return 0;
}
