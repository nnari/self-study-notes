//Needs node.js to run
const {
    performance
} = require('perf_hooks');

const n = 100000;

let characters = "";
for (let i = 0; i < n; i++) {
    characters += Math.floor((Math.random() * 2)).toString();
}

function Onsquared2(n) {
    let counter = 0
    let iters = 0
    for (let i = 0; i < n.length; i++) {
        for (let j = i + 1; j < n.length; j++) {
            iters++
            if (n.charAt(i) === '0' && n.charAt(j) === '1') {
                counter++
            }
        }
    }
}

function Oofn(n) {
    let counter = 0
    let zeros = 0
    for (let i = 0; i < n.length; i++) {
        if (n.charAt(i) === '0')
            zeros++
        else
            counter += zeros
    }
}

var t0 = performance.now()
Oofn(characters);
var t1 = performance.now()
console.log("Call to O(n) algorithm took " + (t1 - t0) + " milliseconds.")

var t2 = performance.now()
Onsquared2(characters);
var t3 = performance.now()
console.log("Call to O(n^2) took " + (t3 - t2) + " milliseconds.")