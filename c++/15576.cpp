#include <iostream>
#include <vector>
#include <complex>

using namespace std;
const double PI = acos(-1);
typedef complex<double> cpx;

vector<int> multiply(vector<cpx>& f, vector<cpx>& g);
void print(vector<int>& arr);

int main()
{
    cin.tie(NULL);
    cout.tie(NULL);
    ios_base::sync_with_stdio(false);

    string str;
    vector<cpx> f, g;

    cin >> str;
    for (int i = (int)str.size() - 1; i >= 0; i--) {
        f.push_back(cpx(str[i] - '0', 0));
    }

    cin >> str;
    for (int i = (int)str.size() - 1; i >= 0; i--) {
        g.push_back(cpx(str[i] - '0', 0));
    }

    vector<int> result = multiply(f, g);

    print(result);

    return 0;
}

void print(vector<int>& arr) {
    for (int i = (int)arr.size() - 1; i >= 0; i--) {
        cout << arr[i];
    }
}

void fast_fourier_transform(vector<cpx>& v, bool is_inverse) {
    size_t len = v.size();

    for (size_t i = 1, j = 0; i < len; i++) {
        size_t bit = len / 2;

        while (j >= bit) {
            j -= bit;
            bit /= 2;
        }

        j += bit;

        if (i < j) {
            swap(v[i], v[j]);
        }
    }

    for (size_t k = 1; k < len; k *= 2) {
        double angle = is_inverse ? PI / k : -PI / k;
        cpx direction(cos(angle), sin(angle));

        for (size_t i = 0; i < len; i += k * 2) {
            cpx unit(1, 0);

            for (size_t j = 0; j < k; j++) {
                cpx even = v[i + j];
                cpx odd = v[i + j + k] * unit;

                v[i + j] = even + odd;
                v[i + j + k] = even - odd;

                unit *= direction;
            }
        }
    }

    if (is_inverse) {
        for (size_t i = 0; i < len; i++) {
            v[i] /= cpx((double)len, 0);
        }
    }
}

vector<int> flatten(vector<cpx>& v) {
    size_t len = v.size();
    vector<int> result(len);
    int carry = 0;

    for (size_t i = 0; i < len; i++) {
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

vector<int> multiply(vector<cpx>& f, vector<cpx>& g) {
    size_t len = 2;

    while (len < f.size() + g.size()) {
        len *= 2;
    }

    f.resize(len);
    g.resize(len);

    fast_fourier_transform(f, false);
    fast_fourier_transform(g, false);

    vector<cpx> w(len);

    for (size_t i = 0; i < w.size(); i++) {
        w[i] = f[i] * g[i];
    }

    fast_fourier_transform(w, true);

    return flatten(w);
}
