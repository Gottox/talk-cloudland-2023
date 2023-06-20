/**
 * @author      : Enno Boland (mail@eboland.de)
 * @file        : main
 * @created     : Monday Jun 19, 2023 17:51:19 CEST
 */

#include <bits/stdc++.h>
#include <cstdint>
using namespace std;

bool
isPrime(uint64_t n) {
    if (n == 1 || n == 0)
        return false;

    for (uint64_t i = 2; i < n; i++) {
        if (n % i == 0)
            return false;
    }
    return true;
}

class Prime {
  private:
    uint64_t value;

  public:
    Prime(uint64_t n) { value = n; }

    uint64_t
    getValue() {
        return value;
    }
};

class PrimeCache {
  private:
    vector<Prime> primes;
    uint64_t next_checked = 0;

  public:
    void
    calculateMorePrimes(uint64_t n) {
        for (; primes.size() < n; next_checked++) {
            uint64_t candidate = next_checked + 1;

            if (isPrime(candidate)) {
                primes.push_back(Prime(candidate));
            }
        }
    }

    Prime *
    getPrime(uint64_t n) {
        calculateMorePrimes(n);

        return &primes[n - 1];
    }
};

// Driver code
int
main() {
    int number;
    PrimeCache cache;
    vector<Prime *> my_primes;

    while (std::cin.good()) {
        cout << "Enter a number: ";
        cin >> number;
        my_primes.push_back(cache.getPrime(number));
        cout << "Prime number " << number << " is "
             << my_primes.back()->getValue() << endl;
        cout << "History: ";
        for (auto prime : my_primes) {
            cout << prime->getValue() << " ";
        }
        cout << endl;
    }

    return 0;
}
