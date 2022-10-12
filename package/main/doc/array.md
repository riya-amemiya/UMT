# UMT Array Object

## List

- [UMT Array Object](#umt-array-object)
  - [List](#list)
  - [arraysJoin](#arraysjoin)
    - [example](#example)
  - [getArrayCommon](#getarraycommon)
    - [example](#example-1)
  - [getArrayDiff](#getarraydiff)
    - [example](#example-2)
  - [quickSort](#quicksort)
    - [example](#example-3)
  - [sum](#sum)
    - [example](#example-4)

## arraysJoin

`arraysJoin` は配列を重複なく結合します。

### example

```js
const arr1 = [1, 2, 3];
const arr2 = [2, 3, 4];
const arr3 = [3, 4, 5];

const result = arraysJoin(arr1, arr2, arr3);
console.log(result); // [1, 2, 3, 4, 5]
```

## getArrayCommon

`getArrayCommon` は配列の共通要素を取得します。

### example

```js
const arr1 = [1, 2, 3];
const arr2 = [2, 3, 4];
const arr3 = [3, 4, 5];

const result = getArrayCommon(arr1, arr2, arr3);
console.log(result); // [3]
```

## getArrayDiff

`getArrayDiff` は配列の差分を取得します。

### example

```js
const arr1 = [1, 2, 3];
const arr2 = [2, 3, 4];
const arr3 = [3, 4, 5];

const result = getArrayDiff(arr1, arr2, arr3);
console.log(result); // [1, 5]
```

## quickSort

`quickSort` はクイックソートを行います。

### example

```js
const arr = [3, 2, 1, 5, 4];

const result = quickSort(arr);
console.log(result); // [1, 2, 3, 4, 5]
```

## sum

`sum` は配列の合計を取得します。

### example

```js
const arr = [1, 2, 3, 4, 5];

const result = sum(arr);
console.log(result); // 15
```
