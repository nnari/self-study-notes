pub fn primesToN(mut n: i32) -> i32 {
    let mut primes = vec![];
    for i in 2..=n {
        primes.push(i);
    }
    for i in 0..primes.len() {
        let cur = primes[i];
        if cur != 0 {
            sieve(&mut primes, cur);
        }
    }

    //count primality
    let mut count = 0;
    for i in 0..primes.len() {
        if primes[i] != 0 {
            count = count + 1;
        }
    }
    count
}

fn sieve(primes: &mut Vec<i32>, factor: i32) {
    for i in 0..primes.len() {
        let value = primes[i];
        if value != 0 && value != factor {
            if value % factor == 0 {
                primes[i] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    
    #![feature(test)]
    extern crate test;
    use super::*;
    use test::Bencher;
    extern crate time;
    use time::PreciseTime;

    #[test]
    fn answer1() {
        let n = 1000;
        assert_eq!(primesToN(n), 168);
    }
    #[test]
    fn answer2() {
        let n = 10000;
        assert_eq!(primesToN(n), 1229);
    }
    #[test]
    fn answer3() {
        let n = 10000;
        assert_eq!(primesToN(n), 9592);
    }

    #[bench]
    fn bench_answer1(b: &mut Bencher) {
        let n = 1000;
        b.iter(|| primesToN(n));
    }
    #[bench]
    fn bench_answer2(b: &mut Bencher) {
        let n = 10000;
        b.iter(|| primesToN(n));
    }
    /* #[bench]
    fn bench_answer3(b: &mut Bencher) {
        let n = 100000;
        b.iter(|| primesToN(n));
    } */
}