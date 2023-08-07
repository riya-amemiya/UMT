const shuffleArray = (array) => {
  const cloneArray = [...array];

  const result = cloneArray.reduce((_, cur, idx) => {
    const rand = Math.floor(Math.random() * (idx + 1));
    cloneArray[idx] = cloneArray[rand];
    cloneArray[rand] = cur;
    return cloneArray;
  });

  return result;
};

const testFunction = (collBackFun) => {
  const arr = [...Array(10000000).keys()];
  return () => collBackFun(shuffleArray(arr));
};
// 足し算
const sum = (x, y) => x + y;
// for++
const date1 = testFunction((arr) => {
  let out = 0;
  for (let i = 0; i < arr.length; i++) {
    out = sum(out, arr[i]);
  }
  return out;
});

// for--
const date2 = testFunction((arr) => {
  let out = 0;
  for (let i = arr.length - 1; i >= 0; i--) {
    out = sum(out, arr[i]);
  }
  return out;
});

// for of
const date3 = testFunction((arr) => {
  let out = 0;
  for (const i of arr) {
    out = sum(out, i);
  }
  return out;
});

// for in
const date4 = testFunction((arr) => {
  let out = 0;
  for (const i in arr) {
    out = sum(out, arr[i]);
  }
  return out;
});

// forEach
const date5 = testFunction((arr) => {
  let out = 0;
  // rome-ignore lint/nursery/noForEach: <explanation>
  arr.forEach((i) => {
    out = sum(out, i);
  });
  return out;
});

// map
const date6 = testFunction((arr) => {
  let out = 0;
  arr.map((i) => {
    out = sum(out, i);
  });
  return out;
});

// speed test
console.time("for++");
date1();
console.timeEnd("for++");

console.time("for--");
date2();
console.timeEnd("for--");

console.time("for of");
date3();
console.timeEnd("for of");

console.time("for in");
date4();
console.timeEnd("for in");

console.time("forEach");
date5();
console.timeEnd("forEach");

console.time("map");
date6();
console.timeEnd("map");
