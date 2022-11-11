#include <iostream>
#include <deque>

using namespace std;

deque<int> addByArray(deque<int>& A, deque<int>& B);
deque<int> multiplyByArray(deque<int>& A, deque<int>& B);
bool isZero(deque<int>& arr);

int main()
{
    string str;
    deque<int> A, B;

    cin >> str;
    for (char c : str) {
        A.push_back(c - '0');
    }

    cin >> str;
    for (char c : str) {
        B.push_back(c - '0');
    }

    deque<int> result = multiplyByArray(A, B);

    if (isZero(result)) {
        cout << 0;
    }
    else {
        for (int c : result) {
            cout << c;
        }
    }

    cout << endl;

    return 0;
}

bool isZero(deque<int>& arr) {
    bool is_zero = true;

    for (size_t i = 0; i < arr.size(); i++) {
        if (arr[i] != 0) {
            return false;
        }
    }

    return is_zero;
}

deque<int> flatten(deque<int>& arr) {
    for (size_t i = arr.size() - 1; i > 0; i--) {
        if (arr[i] < 10) {
            continue;
        }

        arr[i - 1] += arr[i] / 10;
        arr[i] %= 10;
    }

    while (arr[0] >= 10) {
        arr.push_front(arr[0] / 10);
        arr[1] %= 10;
    }

    return arr;
}

deque<int> addByArray(deque<int>& A, deque<int>& B) {
    deque<int> sum;
    size_t longer = max(A.size(), B.size());

    while (A.size() < longer) {
        A.push_front(0);
    }
    while (B.size() < longer) {
        B.push_front(0);
    }

    for (size_t i = 0; i < longer; i++) {
        sum.push_back(A[i] + B[i]);
    }

    return flatten(sum);
}

deque<int> multiplyByArray(deque<int>& A, int B, int radix) {
    deque<int> result;

    for (size_t i = 0; i < A.size(); i++) {
        result.push_back(A[i] * B);
    }

    for (int i = 0; i < radix - 1; i++) {
        result.push_back(0);
    }

    return flatten(result);
}

deque<int> multiplyByArray(deque<int>& A, deque<int>& B) {
    int radix = 1;
    deque<int> sum = { 0 };

    for (int i = (int)B.size() - 1; i >= 0; i--) {
        deque<int> multiply = multiplyByArray(A, B[i], radix);

        sum = addByArray(sum, multiply);
        radix += 1;
    }

    return flatten(sum);
}
