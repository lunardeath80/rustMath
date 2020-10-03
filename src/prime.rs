pub fn prime_sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![1];
    }

    let mut sieve = vec![true; n + 1];
    sieve[0] = false;
    sieve[1] = false;
    let mut div: usize = 2;

    while div < n {
        if !sieve[div] {
            div += 1;
            continue;
        }

        let mut m = 0;

        while m * div + div * div <= n {
            sieve[div * (div + m)] = false;
            m += 1;
        }
        div += 1;
    }

    let mut primes = Vec::with_capacity(n / 2);

    for (i, c) in sieve.iter().enumerate() {
        if *c {
            primes.push(i);
        }
    }

    return primes;
}

pub fn prime_factorise(n: usize) -> Vec<usize> {
    let poss_factors = prime_sieve(n);
    let mut n_tmp = n;

    let mut prime_factorisation = vec![0; poss_factors.len()];

    while n_tmp > 1 {
        for (i, x) in poss_factors.iter().enumerate() {
            if n_tmp % x == 0 {
                n_tmp /= x;

                prime_factorisation[i] += 1;
            }
        }
    }

    return prime_factorisation;
}
