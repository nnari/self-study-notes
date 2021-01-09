//Worst implementation of prime calc I've seen
pub fn primesToN(n: i32) -> i32 {
    let mut count = 0;
    for i in 2..n+1 {
        let mut prime = true;
        for j in 2..i-1 {
            if i%j == 0 {
                prime = false;
                break;
            }
        }
        if prime {
            count = count + 1;
        }
    }
    count
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
        let n = 100000;
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
    #[bench]
    fn bench_answer3(b: &mut Bencher) {
        let n = 100000;
        b.iter(|| primesToN(n));
    }
}