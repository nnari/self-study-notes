# Table of Contents
Author: Tatu Pesonen  
Created on: 09.01.2021  
Last updated on: 09.01.2021  
- [Table of Contents](#table-of-contents)
- [Link to basic exercises](#link-to-basic-exercises)
  - [Exercise 1](#exercise-1)
    - [Solution](#solution)
  - [Exercise 2](#exercise-2)
    - [Solution](#solution-1)

# Link to basic exercises
[https://tasks.withmooc.fi/tira-syksy-2020/1-1](https://tasks.withmooc.fi/tira-syksy-2020)

## Exercise 1
[Link to exercise](https://tasks.withmooc.fi/tira-syksy-2020/1-1)  
Algoritmin toteutus (osa 1)
Positiivinen kokonaisluku on alkuluku, jos se on jaollinen vain 1:llä ja itsellään. Esimerkiksi luvut 3, 7 ja 19 ovat alkulukuja. Luku 10 puolestaan ei ole alkuluku, koska se on jaollinen 2:lla ja 5:llä.
Seuraava algoritmi laskee, montako alkulukua on välillä 2–n. Algoritmi tarkastaa jokaisesta luvusta i, onko se jaollinen jollain luvulla, joka on vähintään 2 ja enintään i–1. Jos tällainen jakaja löytyy, luku ei ole alkuluku, ja muuten luku on alkuluku.
```python
count = 0
for i = 2 to n
    prime = true
    for j = 2 to i-1
        if i%j == 0
            prime = false
            break
    if prime
        count++
print(count)
```
### Solution
[My implementation in Rust](src/exercise1.rs)  
Esimerkiksi kun n = 5, algoritmin tulisi antaa tulos 3, ja kun n = 100, algoritmin tulisi antaa tulos 25.
***

**Results:**  
|    n | Result | Seconds     |
|-----:|--------|-------------|
| 10^3 | 168    | 0.000152336 |
| 10^4 | 1229   | 0.01006521  |
| 10^5 | 9592   | 0.80389643  |


## Exercise 2
[Link to exercise](https://tasks.withmooc.fi/tira-syksy-2020/1-2)  
Algoritmin toteutus (osa 2)
Tehokkaampi tapa etsiä alkulukuja on Eratostheneen seula. Ideana on luoda taulukko, jossa arvo 0 tarkoittaa, että luku on alkuluku, ja arvo 1 tarkoittaa, että luku ei ole alkuluku.
Seuraava toteutus perustuu taulukkoon a, jossa on paikka jokaiselle luvulle väliltä 2–n. Algoritmin alussa taulukon jokainen arvo on 0, ja algoritmi merkitsee arvon 1 luvuille, jotka eivät ole alkulukuja. Sisemmässä silmukassa step i tarkoittaa, että muuttujan arvo kasvaa i:llä joka kierroksella.

```python
count = 0
for i = 2 to n
    if a[i] == 0:
        count++
        for j = 2*i to n step i
            a[j] = 1
print(count)
```
### Solution
[My implementation in Rust](src/exercise2.rs)  
Esimerkiksi kun n = 5, algoritmin tulisi antaa tulos 3, ja kun n = 100, algoritmin tulisi antaa tulos 25.
***

**Results:**  
|    n | Result | Seconds     |
|-----:|--------|-------------|
| 10^3 | 168    | 0.000152336 |
| 10^4 | 1229   | 0.01006521  |
| 10^5 | 9592   | 0.80389643  |


