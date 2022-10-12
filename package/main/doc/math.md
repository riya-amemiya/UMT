# UMT Math Object

## List

- [addition](#addition)
- [average](#average)
- [calculator](#calculator)
- degToRad
- division
- factorial
- factorize
- gcd
- getDecimalLength
- isDouble
- isNumber
- isPrimeNumber
- lcm
- mathConverter
- mathSeparator
- max
- min
- multiples
- multiplication
- nCr
- nHr
- nPr
- primeFactorization
- quotient
- radToDeg
- random
- reduce
- repeatedTrial
- roundOf
- softmax
- standardDeviation
- subtract
- toBinary
- toCelsius
- toKelvin
- valueSwap

## addition

`addition` は加算を行います。

### example

```js
const result = addition(0.1, 0.2);
console.log(result); // 0.3
```

## average

`average` は平均値を取得します。

### example

```js
const result = average(1, 2, 3, 4, 5);
console.log(result); // 3
```

## calculator

`calculator` は計算を行います。

### example

```js
const result = calculator("1 + 2 * 3");
console.log(result); // 7
```

## degToRad

`degToRad` は度数法からラジアンに変換します。

### example

```js
const result = degToRad(180);
console.log(result); // 3.141592653589793
```

## division

`division` は除算を行います。

### example

```js
const result = division(1, 2);
console.log(result); // 0.5
```

## factorial

`factorial` は階乗を取得します。

### example

```js
const result = factorial(5);
console.log(result); // 120
```

## factorize

`factorize` は素因数分解を行います。

### example

```js
const result = factorize(12);
console.log(result); // [2, 2, 3]
```

## gcd

`gcd` は最大公約数を取得します。

### example

```js
const result = gcd(12, 18);
console.log(result); // 6
```

## getDecimalLength

`getDecimalLength` は小数点以下の桁数を取得します。

### example

```js
const result = getDecimalLength(0.1);
console.log(result); // 1
```

## isDouble

`isDouble` は数値が少数かどうかを判定します。

### example

```js
const result = isDouble(0.1);
console.log(result); // true
```

```js
const result = isDouble(1);
console.log(result); // false
```

```js
const result = isDouble("0.1", true);
console.log(result); // true
```

```js
const result = isDouble("0.1", false);
console.log(result); // false
```

## isNumber

`isNumber` は数値かどうかを判定します。

### example

```js
const result = isNumber(1);
console.log(result); // true
```

```js
const result = isNumber("1");
console.log(result); // true
```

```js
const result = isNumber("1", false);
console.log(result); // false
```

## isPrimeNumber

`isPrimeNumber` は素数かどうかを判定します。

### example

```js
const result = isPrimeNumber(7);
console.log(result); // true
```

## lcm

`lcm` は最小公倍数を取得します。

### example

```js
const result = lcm(12, 18);
console.log(result); // 36
```

## mathConverter

`mathConverter` は数式を変換します。

### example

```js

// mathConverter("x^2") => (x + y)(x - y) + y ^ 2

const result = mathConverter("17^2");
console.log(result); // 24 * 10 + 7 * 7
```

## mathSeparator

`mathSeparator` は数値を最大桁数とそれ以外に分割します。

### example

```js
const result = mathSeparator(110000);
console.log(result); // [100000, 10000]
```

## max

`max` は最大値を取得します。

### example

```js
const result = max(1, 2, 3, 4, 5);
console.log(result); // 5
```

## min

`min` は最小値を取得します。

### example

```js
const result = min(1, 2, 3, 4, 5);
console.log(result); // 1
```

## multiples

`multiples` は倍数を取得します。

### example

```js
const result = multiples(2, 10);
console.log(result); // [2, 4, 6, 8, 10]
```

## multiplication

`multiplication` は乗算を行います。

### example

```js
const result = multiplication(1.2, 2.11);
console.log(result); // 2.532
```

## nCr

`nCr` は組み合わせを取得します。

### example

```js
const result = nCr(5, 3);
console.log(result); // 10
```

## nPr

`nPr` は順列を取得します。

### example

```js
const result = nPr(5, 3);
console.log(result); // 60
```

## primeFactorization

`primeFactorization` は素因数分解を行います。

### example

```js
const result = primeFactorization(12);
console.log(result); // [2, 2, 3]
```

## quotient

`quotient` は商を取得します。

### example

```js
const result = quotient(1.1, 2.11);
console.log(result); // 0.5194805194805194
```

## radToDeg

`radToDeg` はラジアンから度数法に変換します。

### example

```js
const result = radToDeg(Math.PI);
console.log(result); // 180
```

## random

`random` は乱数を取得します。

### example

```js
const result = random(1, 10);
console.log(result);
```

## redouce

`redouce` は分数を約分します。

### example

```js
const result = redouce(1, 2);
console.log(result); // [1, 2]
```

## repeatedTrial

`repeatedTrial` は重複試行を行います。

### example

```js
const result = repeatedTrial(4, 2, { x: 1, y: 3 });
console.log(result); // [8, 27]
```

## roundOff

`roundOff` は四捨五入を行います。

### example

```js
const result = roundOff(0.1234, 2);
console.log(result); // 0.12
```

```js
const result = roundOff(0.1234, 3);
console.log(result); // 0.123
```

## softMax

`softMax` はソフトマックス関数を取得します。

### example

```js
const result = softMax([1, 2, 3]);
console.log(result); // [0.09003057317038046, 0.24472847105479764, 0.6652409557748219]
```

## standardDeviation

`standardDeviation` は標準偏差を取得します。

### example

```js
const result = standardDeviation([1, 2, 3, 4, 5]);
console.log(result); // 1.4142135623730951
```

## subtract

`subtract` は減算を行います。

### example

```js
const result = subtract(1.1, 2.11);
console.log(result); // -1.01
```

## toBinary

`toBinary` は10進数から2進数に変換します。

### example

```js
const result = toBinary(10);
console.log(result); // 1010
```

## toCelsius

`toCelsius` は絶対温度から摂氏温度に変換します。

### example

```js
const result = toCelsius(273.15);
console.log(result); // 0
```

## toKelvin

`toKelvin` は摂氏から絶対温度に変換します。

### example

```js
const result = toKelvin(0);
console.log(result); // 273.15
```

## valueSwap

`valueSwap` は値を入れ替えます。

### example

```js
const result = valueSwap(1, 2);
console.log(result); // [2, 1]
```
