# UMT Date Object

## List

- [now](#now)

## now

時差を設定しておくことで、サーバーの場所に関係なく現在時刻を取得できます。
UTCが基準です。
デフォルトは+9時間。

### example

```js
const result = now();
console.log(result);
```

```js
const result = now(9);
console.log(result);
```
