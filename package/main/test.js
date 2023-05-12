const { map } = require("./module/Array/map");
const inisalArray = Array.from({ length: 1000000 }, (_, i) => i + 1);
const inisalObject = Object.fromEntries(
	Array.from({ length: 1000000 }, (_, i) => [i, i + 1]),
);
const callbackfn = (item) => item * 2;
console.log("====================================");
console.time("dmap:array");
const arr1 = [...inisalArray];
const newArr1 = arr1.map(callbackfn);
console.log(newArr1.length);
console.timeEnd("dmap:array");

console.time("dmap:object");
const obj1 = { ...inisalObject };
const newObj1 = Object.keys(obj1).map(callbackfn);
console.log(newObj1.length);
console.timeEnd("dmap:object");
console.log("====================================");

console.time("map:array");
const arr2 = [...inisalArray];
const newArr2 = map(arr2, callbackfn);
console.log(newArr2.length);
console.timeEnd("map:array");

console.time("map:object");
const obj2 = { ...inisalObject };
const newObj2 = map(obj2, callbackfn);
console.log(newObj2.length);
console.timeEnd("map:object");

console.log("====================================");
console.time("for:array");
const arr3 = [...inisalArray];
const newArr3 = [];
for (let i = 0; i < arr3.length; i++) {
	newArr3.push(callbackfn(arr3[i]));
}
console.log(newArr3.length);
console.timeEnd("for:array");

console.time("for:object");
const obj3 = { ...inisalObject };
const newObj3 = [];
for (const key in obj3) {
	newObj3.push(callbackfn(obj3[key]));
}
console.log(newObj3.length);
console.timeEnd("for:object");
console.log("====================================");

console.time("for of:array");
const arr4 = [...inisalArray];
const newArr4 = [];
for (const item of arr4) {
	newArr4.push(callbackfn(item));
}
console.log(newArr4.length);
console.timeEnd("for of:array");

console.time("for of:object");
const obj4 = { ...inisalObject };
const newObj4 = [];
for (const value of Object.values(obj4)) {
	newObj4.push(callbackfn(value));
}
console.log(newObj4.length);
console.timeEnd("for of:object");

console.log("====================================");
console.time("for in:array");
const arr5 = [...inisalArray];
const newArr5 = [];
for (const index in arr5) {
	newArr5.push(callbackfn(arr5[index]));
}

console.log(newArr5.length);
console.timeEnd("for in:array");
console.time("for in:object");
const obj5 = { ...inisalObject };
const newObj5 = [];
for (const key in obj5) {
	newObj5.push(callbackfn(obj5[key]));
}
console.log(newObj5.length);
console.timeEnd("for in:object");

console.log("====================================");
console.time("forEach:array");
const arr6 = [...inisalArray];
const newArr6 = [];
arr6.forEach((item) => {
	newArr6.push(callbackfn(item));
});
console.log(newArr6.length);
console.timeEnd("forEach:array");

console.time("forEach:object");
const obj6 = { ...inisalObject };
const newObj6 = [];
Object.values(obj6).forEach((value) => {
	newObj6.push(callbackfn(value));
});
console.log(newObj6.length);
console.timeEnd("forEach:object");
