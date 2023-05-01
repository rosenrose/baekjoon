#include <iostream>
#include <iomanip>
#include <vector>
#include <complex>

using namespace std;
const double PI = acos(-1);
const uint64_t POW = 1000;
typedef complex<double> cpx;

vector<uint16_t> multiply(vector<cpx>& f, vector<cpx>& g);
void print(vector<uint16_t>& arr);

int main() {
    cin.tie(NULL);
    cout.tie(NULL);
    ios_base::sync_with_stdio(false);

    string str;
    vector<cpx> f, g;

    cin >> str;
    for (int i = str.size() - 1; i >= 0; i -= 3) {
        uint16_t num = 0;

        for (int j = max(i - 2, 0); j <= i; j++) {
            num = num * 10 + (str[j] - '0');
        }

        f.push_back(cpx(num, 0));
    }

    cin >> str;
    for (int i = str.size() - 1; i >= 0; i -= 3) {
        uint16_t num = 0;

        for (int j = max(i - 2, 0); j <= i; j++) {
            num = num * 10 + (str[j] - '0');
        }

        g.push_back(cpx(num, 0));
    }

    vector<uint16_t> result = multiply(f, g);

    print(result);

    return 0;
}

void print(vector<uint16_t>& arr) {
    for (int i = arr.size() - 1; i >= 0; i--) {
        if (i == arr.size() - 1) {
            cout << arr[i];
            cout << setfill('0');
        } else {
            cout << setw(3) << arr[i];
        }
    }
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

vector<uint16_t> normalize(vector<cpx>& v) {
    size_t len = v.size();
    vector<uint16_t> result(len);
    uint64_t carry = 0;

    for (auto i = 0; i < len; i++) {
        uint64_t num = carry + (uint64_t)round(v[i].real());
        carry = num / POW;
        result[i] = num % POW;
    }

    if (carry > 0) {
        result.push_back(carry);
    }

    while (result.size() > 1 && result.back() == 0) {
        result.pop_back();
    }

    return result;
}

vector<uint16_t> multiply(vector<cpx>& f, vector<cpx>& g) {
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
