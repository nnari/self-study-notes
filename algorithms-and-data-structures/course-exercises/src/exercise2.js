const { performance } = require('perf_hooks');

const countPrimesGood = (n) => {
    let count = 0
    let a = new Array(n).fill(true)
    for (let i = 2; i < n - 1; i++) {
        if (a[i]) {
            count++
            for (let j = i * i; j < n; j += i) {
                a[j] = false
            }
        }
    }
    return count
}

const countPrimesBad = (n) => {
    let count = 0, prime = true;
    for (let i = 2; i < n; i++) {
        for (let j = 2; j < i - 1; j++) {
            if (i % j == 0) {
                prime = true;
                break;
            }
        }
        if (prime)
            count++;
    }
    return count
}


let t1 = performance.now()
let count = countPrimesGood(100000);
let t2 = performance.now();
console.log("Good prime calc function time in ms: " + (t2 - t1))
let t3 = performance.now()
let count2 = countPrimesBad(100000);
let t4 = performance.now();
console.log("Bad prime calc function time in ms: " + (t4 - t3))