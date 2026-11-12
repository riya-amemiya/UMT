const INITIAL_CAPACITY = 16;

/**
A double-ended queue backed by a growable circular buffer.
Elements can be added and removed at both ends.

## Features
- **pushFront(value)**: Add element to the front
- **pushBack(value)**: Add element to the back
- **popFront()**: Remove and return the front element
- **popBack()**: Remove and return the back element
- **peekFront()**: View the front element without removing
- **peekBack()**: View the back element without removing
- **size**: Get number of elements
- **isEmpty**: Check if deque is empty
- **clear()**: Remove all elements
- **[Symbol.iterator]**: Iterate from front to back

## Time Complexity
- pushFront / pushBack: O(1) amortized
- popFront / popBack: O(1)
- peekFront / peekBack: O(1)
- size / isEmpty: O(1)

@example
```typescript
const deque = new Deque<number>();
deque.pushBack(2);
deque.pushBack(3);
deque.pushFront(1);

console.log([...deque]); // [1, 2, 3]
console.log(deque.popFront()); // 1
console.log(deque.popBack()); // 3
console.log(deque.size); // 1
```

@template T - The type of elements stored in the deque
*/
export class Deque<T> {
  private buffer: (T | undefined)[] = new Array(INITIAL_CAPACITY);
  private head = 0;
  private length = 0;

  /**
  Doubles the buffer capacity, moving the elements to the start of the
  new buffer in front-to-back order.
  */
  private grow(): void {
    const capacity = this.buffer.length;
    const next = new Array<T | undefined>(capacity * 2);
    for (let index = 0; index < this.length; index++) {
      next[index] = this.buffer[(this.head + index) & (capacity - 1)];
    }
    this.buffer = next;
    this.head = 0;
  }

  /**
  Returns the number of elements in the deque.
  @returns The number of elements in the deque
  @example
  ```typescript
  const deque = new Deque<string>();
  deque.pushBack("a");
  console.log(deque.size); // 1
  ```
  */
  get size(): number {
    return this.length;
  }

  /**
  Checks if the deque is empty.
  @returns True if the deque is empty, false otherwise
  @example
  ```typescript
  const deque = new Deque<string>();
  console.log(deque.isEmpty); // true
  deque.pushBack("a");
  console.log(deque.isEmpty); // false
  ```
  */
  get isEmpty(): boolean {
    return this.length === 0;
  }

  /**
  Adds an element to the front of the deque.
  @param value - The value to add
  @example
  ```typescript
  const deque = new Deque<number>();
  deque.pushFront(2);
  deque.pushFront(1);
  console.log(deque.peekFront()); // 1
  ```
  */
  pushFront(value: T): void {
    if (this.length === this.buffer.length) {
      this.grow();
    }
    this.head = (this.head - 1 + this.buffer.length) & (this.buffer.length - 1);
    this.buffer[this.head] = value;
    this.length += 1;
  }

  /**
  Adds an element to the back of the deque.
  @param value - The value to add
  @example
  ```typescript
  const deque = new Deque<number>();
  deque.pushBack(1);
  deque.pushBack(2);
  console.log(deque.peekBack()); // 2
  ```
  */
  pushBack(value: T): void {
    if (this.length === this.buffer.length) {
      this.grow();
    }
    this.buffer[(this.head + this.length) & (this.buffer.length - 1)] = value;
    this.length += 1;
  }

  /**
  Removes and returns the front element.
  @returns The front element, or undefined if the deque is empty
  @example
  ```typescript
  const deque = new Deque<number>();
  deque.pushBack(1);
  deque.pushBack(2);
  console.log(deque.popFront()); // 1
  console.log(deque.popFront()); // 2
  console.log(deque.popFront()); // undefined
  ```
  */
  popFront(): T | undefined {
    if (this.length === 0) {
      return undefined;
    }
    const value = this.buffer[this.head];
    this.buffer[this.head] = undefined;
    this.head = (this.head + 1) & (this.buffer.length - 1);
    this.length -= 1;
    return value;
  }

  /**
  Removes and returns the back element.
  @returns The back element, or undefined if the deque is empty
  @example
  ```typescript
  const deque = new Deque<number>();
  deque.pushBack(1);
  deque.pushBack(2);
  console.log(deque.popBack()); // 2
  console.log(deque.popBack()); // 1
  console.log(deque.popBack()); // undefined
  ```
  */
  popBack(): T | undefined {
    if (this.length === 0) {
      return undefined;
    }
    const index = (this.head + this.length - 1) & (this.buffer.length - 1);
    const value = this.buffer[index];
    this.buffer[index] = undefined;
    this.length -= 1;
    return value;
  }

  /**
  Returns the front element without removing it.
  @returns The front element, or undefined if the deque is empty
  @example
  ```typescript
  const deque = new Deque<number>();
  deque.pushBack(1);
  deque.pushBack(2);
  console.log(deque.peekFront()); // 1
  console.log(deque.size); // 2
  ```
  */
  peekFront(): T | undefined {
    return this.length === 0 ? undefined : this.buffer[this.head];
  }

  /**
  Returns the back element without removing it.
  @returns The back element, or undefined if the deque is empty
  @example
  ```typescript
  const deque = new Deque<number>();
  deque.pushBack(1);
  deque.pushBack(2);
  console.log(deque.peekBack()); // 2
  console.log(deque.size); // 2
  ```
  */
  peekBack(): T | undefined {
    return this.length === 0
      ? undefined
      : this.buffer[(this.head + this.length - 1) & (this.buffer.length - 1)];
  }

  /**
  Removes all elements from the deque.
  @example
  ```typescript
  const deque = new Deque<number>();
  deque.pushBack(1);
  deque.clear();
  console.log(deque.size); // 0
  ```
  */
  clear(): void {
    this.buffer = new Array(INITIAL_CAPACITY);
    this.head = 0;
    this.length = 0;
  }

  /**
  Iterates over the elements from front to back.
  @returns An iterator over the elements
  @example
  ```typescript
  const deque = new Deque<number>();
  deque.pushBack(2);
  deque.pushFront(1);
  console.log([...deque]); // [1, 2]
  ```
  */
  *[Symbol.iterator](): IterableIterator<T> {
    for (let index = 0; index < this.length; index++) {
      yield this.buffer[(this.head + index) & (this.buffer.length - 1)] as T;
    }
  }
}
