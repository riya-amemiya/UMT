const { arrayMap } = require("./module/Array/arrayMap");
const { objectMap } = require("./module/Object/objectMap");
const inisalArray = Array.from({ length: 1000000 }, (_, i) => i + 1);
const inisalObject = Object.fromEntries(
  Array.from({ length: 1000000 }, (_, i) => [i, i + 1]),
);
const callbackfn = (item) => {
  let n = item ** 2;
  while (n !== 1) {
    if (n % 2 === 0) {
      n = n / 2;
    } else {
      n = n * 3 + 1;
    }
  }
  return n;
};
const answer = inisalArray.map(callbackfn);
console.log("====================================");
console.time("dmap:array");
const arr1 = [...inisalArray];
const newArr1 = arr1.map(callbackfn);
console.log(newArr1.toString() === answer.toString());
console.timeEnd("dmap:array");

console.time("dmap:object");
const obj1 = { ...inisalObject };
const newObj1 = Object.values(obj1).map(callbackfn);
console.log(newObj1.toString() === answer.toString());
console.timeEnd("dmap:object");
console.log("====================================");

console.time("map:array");
const arr2 = [...inisalArray];
const newArr2 = arrayMap(arr2, callbackfn);
console.log(newArr2.toString() === answer.toString());
console.timeEnd("map:array");

console.time("map:object");
const obj2 = { ...inisalObject };
const newObj2 = objectMap(obj2, callbackfn);
console.log(newObj2.toString() === answer.toString());
console.timeEnd("map:object");

console.log("====================================");
