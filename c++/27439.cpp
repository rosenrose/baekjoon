#include <iostream>
#include <iomanip>
#include <vector>
#include <complex>

using namespace std;
const double PI = acos(-1);
const uint64_t POW = 1000;
typedef complex<double> cpx;

vector<cpx> multiply(vector<cpx>& f, vector<cpx>& g);
vector<cpx> factorial(__uint128_t start, __uint128_t end);
void print(vector<cpx>& arr);

int main() {
    cin.tie(NULL);
    cout.tie(NULL);
    ios_base::sync_with_stdio(false);

    int n;
    cin >> n;

    vector<cpx> result = factorial(0, (__uint128_t)n);
    print(result);

    return 0;
}

void print(vector<cpx>& arr) {
    for (int i = arr.size() - 1; i >= 0; i--) {
        if (i == arr.size() - 1) {
            cout << arr[i].real();
            cout << setfill('0');
        } else {
            cout << setw(3) << arr[i].real();
        }
    }
}

vector<cpx> factorial(__uint128_t start, __uint128_t end) {
    if (end - start <= 6) {
        __uint128_t product = 1;
        vector<cpx> result;

        for (auto num = (start == 0) ? 1 : start; num <= end; num++) {
            product *= num;
        }

        while (product > 0) {
            result.push_back(cpx(product % POW, 0));
            product /= POW;
        }

        return result;
    }

    __uint128_t mid = (start + end) >> 1;
    vector<cpx> lower = factorial(start, mid);
    vector<cpx> upper = factorial(mid + 1, end);

    return multiply(lower, upper);
}

void fast_fourier_transform(vector<cpx>& v, bool is_inverse) {
    size_t len = v.size();

    for (size_t i = 1, j = 0; i < len; i++) {
        auto bit = len >> 1;

        while (j >= bit) {
            j -= bit;
            bit >>= 1;
        }

        j += bit;

        if (i < j) {
            swap(v[i], v[j]);
        }
    }

    double direction = is_inverse ? -1.0 : 1.0;

    for (auto k = 2; k <= len; k <<= 1) {
        double angle = -2.0 * PI * direction / k;
        cpx wlen(cos(angle), sin(angle));

        for (auto i = 0; i < len; i += k) {
            cpx w(1, 0);
            auto half = k >> 1;

            for (auto j = 0; j < half; j++) {
                cpx even = v[i + j];
                cpx odd = v[i + j + half];

                v[i + j] = even + odd * w;
                v[i + j + half] = even - odd * w;

                w *= wlen;
            }
        }
    }

    if (is_inverse) {
        for (auto& i : v) {
            i /= cpx(len, 0);
        }
    }
}

vector<cpx> normalize(vector<cpx>& v) {
    size_t len = v.size();
    vector<uint16_t> temp(len);
    uint64_t carry = 0;

    for (auto i = 0; i < len; i++) {
        uint64_t num = carry + (uint64_t)round(v[i].real());
        carry = num / POW;
        temp[i] = num % POW;
    }

    if (carry > 0) {
        temp.push_back(carry);
    }

    while (temp.size() > 1 && temp.back() == 0) {
        temp.pop_back();
    }
    
    vector<cpx> result(temp.size());
    
    for (auto i = 0; i < temp.size(); i++) {
        result[i] = cpx(temp[i], 0);
    }

    return result;
}

vector<cpx> multiply(vector<cpx>& f, vector<cpx>& g) {
    size_t len = 2;

    while (len < f.size() + g.size()) {
        len <<= 1;
    }

    f.resize(len);
    g.resize(len);

    fast_fourier_transform(f, false);
    fast_fourier_transform(g, false);

    vector<cpx> w(len);

    for (auto i = 0; i < w.size(); i++) {
        w[i] = f[i] * g[i];
    }

    fast_fourier_transform(w, true);

    return normalize(w);
}
