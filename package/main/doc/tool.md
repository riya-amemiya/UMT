# UMT Tool Object

## List

- [UMT Tool Object](#umt-tool-object)
  - [List](#list)
  - [birthday](#birthday)
    - [example](#example)
  - [dayOfWeek](#dayofweek)
    - [example](#example-1)
  - [pipeFunction](#pipefunction)
    - [example](#example-2)

## birthday

生年月日から年齢を計算します。

### example

```js
const result = birthday(2000, 1, 1);
console.log(result);
```

## dayOfWeek

曜日を取得します。

### example

```js
const result = dayOfWeek(2019, 1, 1);
console.log(result);
```

## pipeFunction

関数をパイプでつなげて実行します。

### example

```js
const result = pipeFunction((x) => x + 1)(x => x(2));
console.log(result); // 3
```
