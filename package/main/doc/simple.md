# UMT Simple Object

より簡単に使えるようにしたオブジェクトです。

## List

- [UMT Simple Object](#umt-simple-object)
  - [List](#list)
  - [Date](#date)
    - [now](#now)
      - [example](#example)
  - [Math](#math)
    - [dayOfWeek](#dayofweek)
      - [example](#example-1)
    - [deviationValueSimple](#deviationvaluesimple)
      - [example](#example-2)
  - [Tool](#tool)
    - [birthday](#birthday)
      - [example](#example-3)

## Date

### now

現在時刻を取得します。

#### example

```js
const result = now();
console.log(result);
```

```js
const result = now(9);
console.log(result);
```

```js
const result = now("9");
console.log(result);
```

## Math

### dayOfWeek

曜日を取得します。

#### example

```js
const result = dayOfWeek(2019, 1, 1);
console.log(result);
```

```js
const result = dayOfWeek('2019-01-01');
console.log(result);
```

```js
const result = dayOfWeek('2019/01/01');
console.log(result);
```

```js
const result = dayOfWeek('2019:01:01');
console.log(result);
```

### deviationValueSimple

偏差値を計算します。

#### example

```js
const result = deviationValueSimple(100, 50, 10); // 100点の試験で50点を取った人の偏差値
console.log(result);
```

```js
const result = deviationValueSimple(100, [50, 60, 70, 80, 90, 100]); //平均点が50点、標準偏差が10点の試験で100点を取った人の偏差値
console.log(result);
```

## Tool

### birthday

生年月日から年齢を計算します。

#### example

```js
const result = birthday(2000, 1, 1);
console.log(result);
```

```js
const result = birthday('2000-01-01');
console.log(result);
```

```js
const result = birthday('2000/01/01');
console.log(result);
```

```js
const result = birthday('2000:01:01');
console.log(result);
```
