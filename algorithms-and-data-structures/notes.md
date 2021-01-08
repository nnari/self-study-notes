# Table of Contents
- [Table of Contents](#table-of-contents)
  - [Links to course material](#links-to-course-material)
- [Book](#book)
  - [Introduction](#introduction)
  - [Time complexity](#time-complexity)
    - [Example usage of calculating functions efficiency](#example-usage-of-calculating-functions-efficiency)
    - [Some information](#some-information)
  - [Equations to calculate time complexity](#equations-to-calculate-time-complexity)
    - [Time complexity for algorithms with loops is $O(n^k)$](#time-complexity-for-algorithms-with-loops-is-onk)
  - [Common time complexities](#common-time-complexities)
    - [Constant time](#constant-time)
    - [Logarithmic](#logarithmic)
    - [Linear](#linear)
    - [Sorting](#sorting)
    - [Quadratic](#quadratic)
    - [Some other time complexities](#some-other-time-complexities)

## Links to course material
[Tietorakenteet ja algoritmit syksy 2020](https://tira.mooc.fi/syksy-2020/pages/materiaali.html)

# Book

## Introduction
> "Using these basic building blocks of programming, any algorithm can be created."
> 
> "An algorithm can be considered *good* if it outputs fast with given large input (so the n is big!)"

List of programming constructs that the quote refers to:
* Variables
* Conditional statements
* Loops
* Arrays (Lists)
* Functions (called procedures for functions with no return type)
    * Recursion, but this is a part of functions

## Time complexity

### Example usage of calculating functions efficiency

```
1 laskuri = 0
2 for i = 0 to n-1 //n
3   if luvut[i] == x //2n
4       laskuri += 1 //3n
```
Given the following algorithm, the efficiency is as follows:  
Best case: 2n + 1
Worst case: 3n + 1

According to the book, this kind of algorithm analysis is more precise than we need here.  
You may occasionally need more accurate analysis methods, though.

### Some information
n = input size, for example length of an array. (in Big O notation)  
 
**O(n) = All functions that don't conditionally branch and don't contain loops are atleast O(n).**

## Equations to calculate time complexity

**Single commands**  
If the code doesn't have loops but only commands, time complexity is O(1).  
Example:
```
c = a+b
if c >= 0
    print(c)
```
Code that has a time complexity of O(1) is often left out with `...`

**Loops**  
k = Amount of nested loops. In this example, time complexity is $O(n^2)$
```js
for (let i = 0; i < n; i++) {
    for (let j = 0; i < n; j++) {
        ...
    }
}
```
### Time complexity for algorithms with loops is $O(n^k)$
You can use the following formula to calculate time complexity for [quadratic](#Quadratic) and [cubic](#Cubic) algorithms aswell.
Where n is the input size and k is the amount of nested loops.

**Algorithms one after another**
Code segments after each other have the time complexity of the function with largest time complexity in the algorithm.

```python
for i = 1 to n #O(n)
...
for i = 1 to n
    for j = 1 to n #O(n^2)
...
for i = 1 to n #O(n)
```
So the time complexity for this algorithm is $O(n^2)$ because it is the segment with the largest time complexity.

**Multiple variables**
```js
for(let i = 1; i < n; i++) {
    for(let j = 1; i < m; j++) {
        ...
    }
}
```
Time complexity for this function is $O(nm)$

**Recursive algorithms**
```js
function f(n) {
    if (n === 1) return
    f(n-1)
}
```
Function is called $n$ times and each call takes $O(1)$ amount of time.  
The time complexity of the recursive algorithm is calculated by multiplying those values:  
$O(1) * O(n) = O(n)$, so the function time complexity is $O(n)$.
***
Example with another recursive algorithm 
```js
function g(n) {
    if (n === 1) return
    g(n-1)
    g(n-1)
}
```
Calling this function would result in the following time complexity:
$$
1+2+4+...+2^{n-1}=2^n-1
$$
Each call takes $O(1)$ of time so the time complexity is $O(1) * O(2n) = O(2^n)$

## Common time complexities

### Constant time
$O(1)$ (constant time)  
In $O(1)$ algorithms, the size of input doesn't affect algorithm speed.
```js
const sumOfNumbersBeforeN = n => n*(n+1)/2 //15 given n of 5
```

### Logarithmic
$O(\log n)$ (logarithmic)  
A logarithmic algorithm usually halves the input on each step.
```js
let counter = 0;
while (n >= 1) {
    counter++
    n = n / 2
}
```
$\log{{_2}^n}$ algorithm
> "An important factor related to logarithms is that $\log n$ is a small number, whereas $n$ is any number that usually shows up in algorithms."

Logarithmic functions halve input on each iteration, so they are efficient.

### Linear
$O(n)$
```js
let sum = 0
let arr = [...]; //arr.length is size of n
for(let i = 0; i < arr.length; i++) {
    sum = sum + arr[i]
}
```
Given an array of size $n$, linear time complexity is often the best that can be achieved.  
This is because the algorithm has to check the input at least once before it can return a value.

### Sorting
$O({n}\log{n})$
```js
let array1 = [...] //This function is O(n log n)
let same = false
array1.sort() //Sorting the array here
for(let i = 0; i < array1.length; i++) {    |
    if(array1[i] === array1[i-1])           | //This section is O(n)
        same = true                         |
}
```
This example sorts the array first with a $O({n}\log{n})$ time complexity sort algorithm so all the indices are next to each other.  
**Remember that the time complexity of an algorithm is the largest time complexity "chunk" in that algorithm.**

### Quadratic
```js
let arr = [...]
let x = ?
let ok = false
let iters = 0;
for(let i = 0; i < arr.length; i++) {
    for(j = i+1; j < arr.length; j++) {
        if(arr[i] + arr[j] === x)
            ok = true
            iters++
    }
}
console.log(iters, ok)
```
To calculate the time complexity of quadratic & cubic algorithms you can use [$O(n^k)$](#time-complexity-for-algorithms-with-loops-is-onk).

### Some other time complexities
![](imgs/rest-of-the-complexities.png)