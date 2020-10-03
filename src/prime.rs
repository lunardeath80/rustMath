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
