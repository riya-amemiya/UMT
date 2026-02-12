import { lazyTake } from "./package/main/src/Iterator/lazyTake";

function* source() {
  yield 1;
  yield 2;
  yield 3;
}

const gen = source();
const taken = lazyTake(gen, 0);
console.log([...taken]); // []
console.log(gen.next()); // { value: 1, done: false } if 0 was consumed? No, if 1 was consumed, next is 2.
