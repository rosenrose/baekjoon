#include <iostream>
#include <vector>
#include <complex>

using namespace std;
const double PI = acos(-1);
typedef complex<double> cpx;

vector<int> multiply(vector<int>& f, vector<int>& g);
vector<int> factorial(int64_t start, int64_t end);
void print(vector<int>& arr);

int main() {
    cin.tie(NULL);
    cout.tie(NULL);
    ios_base::sync_with_stdio(false);

    int64_t n;

    cin >> n;
    vector<int> result = factorial(0, n);

    print(result);

    return 0;
}

void print(vector<int>& arr) {
    for (auto i = arr.rbegin(); i != arr.rend(); i++) {
        cout << *i;
    }
}

vector<int> factorial(int64_t start, int64_t end) {
    if (end - start <= 2) {
        int64_t product = 1;
        vector<int> result;

        for (auto num = (start == 0) ? 1 : start; num <= end; num++) {
            product *= num;
        }

        while (product > 0) {
            result.push_back(product % 10);
            product /= 10;
        }

        return result;
    }

    int64_t mid = (start + end) / 2;
    vector<int> lower = factorial(start, mid);
    vector<int> upper = factorial(mid + 1, end);

    return multiply(lower, upper);
}

void fast_fourier_transform(vector<cpx>& v, bool is_inverse) {
    size_t len = v.size();

    for (size_t i = 1, j = 0; i < len; i++) {
        auto bit = len / 2;

        while (j >= bit) {
            j -= bit;
            bit /= 2;
        }

        j += bit;

        if (i < j) {
            swap(v[i], v[j]);
        }
    }

    for (auto k = 1; k < len; k *= 2) {
        double angle = is_inverse ? PI / k : -PI / k;
        cpx direction(cos(angle), sin(angle));

        for (auto i = 0; i < len; i += k * 2) {
            cpx unit(1, 0);

            for (auto j = 0; j < k; j++) {
                cpx even = v[i + j];
                cpx odd = v[i + j + k] * unit;

                v[i + j] = even + odd;
                v[i + j + k] = even - odd;

                unit *= direction;
            }
        }
    }

    if (is_inverse) {
        for (auto& i : v) {
            i /= cpx((double)len, 0);
        }
    }
}

vector<int> flatten(vector<cpx>& v) {
    size_t len = v.size();
    vector<int> result(len);
    int carry = 0;

    for (auto i = 0; i < len; i++) {
        result[i] = carry + (int)round(v[i].real());

        carry = result[i] / 10;

        if (result[i] >= 10) {
            result[i] %= 10;
        }
    }

    if (carry > 0) {
        result.push_back(carry);
    }

    while (result.size() > 1 && result.back() == 0) {
        result.pop_back();
    }

    return result;
}

vector<int> multiply(vector<int>& a, vector<int>& b) {
    size_t len = 2;

    while (len < a.size() + b.size()) {
        len *= 2;
    }

    vector<cpx> f(len), g(len);

    for (auto i = 0; i < max(a.size(), b.size()); i++) {
        if (i < a.size()) {
            f[i] = cpx(a[i], 0);
        }
        if (i < b.size()) {
            g[i] = cpx(b[i], 0);
        }
    }

    fast_fourier_transform(f, false);
    fast_fourier_transform(g, false);

    vector<cpx> w(len);

    for (auto i = 0; i < w.size(); i++) {
        w[i] = f[i] * g[i];
    }

    fast_fourier_transform(w, true);

    return flatten(w);
}
