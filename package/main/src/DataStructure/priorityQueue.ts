/**
 * A priority queue implementation using a binary heap.
 * Higher priority values are dequeued first.
 * 
 * ## Features
 * - **enqueue(value, priority)**: Add element with priority
 * - **enqueueBack(value)**: Add element to the back with lowest priority
 * - **dequeue()**: Remove and return highest priority element
 * - **peek()**: View highest priority element without removing
 * - **peekPriority()**: View highest priority value
 * - **size**: Get number of elements
 * - **isEmpty**: Check if queue is empty
 * - **clear()**: Remove all elements
 * - **toArray()**: Get all elements as array
 * - **toArrayWithPriorities()**: Get all elements with priorities
 * 
 * ## Time Complexity
 * - enqueue: O(log n)
 * - dequeue: O(log n)
 * - peek: O(1)
 * - peekPriority: O(1)
 * 
 * @example
 * ```typescript
 * const queue = new PriorityQueue<string>();
 * queue.enqueue("low", 1);
 * queue.enqueue("high", 3);
 * queue.enqueue("medium", 2);
 * 
 * console.log(queue.dequeue()); // "high"
 * console.log(queue.dequeue()); // "medium"
 * console.log(queue.dequeue()); // "low"
 * ```
 * 
 * @example
 * ```typescript
 * // Initialize with elements
 * const queue = new PriorityQueue([
 *   { value: "task1", priority: 1 },
 *   { value: "task2", priority: 3 },
 *   { value: "task3", priority: 2 }
 * ]);
 * 
 * console.log(queue.peek()); // "task2"
 * console.log(queue.peekPriority()); // 3
 * ```
 * 
 * @template T - The type of elements stored in the queue
 */
export class PriorityQueue<T> {
  private heap: Array<{ value: T; priority: number }> = [];

  /**
   * Creates a new PriorityQueue instance.
   * @param initialElements - Optional array of initial elements with priorities
   * @example
   * ```typescript
   * const queue = new PriorityQueue<string>();
   * // or
   * const queue = new PriorityQueue([
   *   { value: "item1", priority: 10 },
   *   { value: "item2", priority: 5 }
   * ]);
   * ```
   */
  constructor(initialElements?: Array<{ value: T; priority: number }>) {
    if (initialElements) {
      this.heap = [...initialElements];
      this.buildHeap();
    }
  }

  /**
   * Returns the number of elements in the queue.
   * @returns The number of elements in the queue
   * @example
   * ```typescript
   * const queue = new PriorityQueue<string>();
   * queue.enqueue("item", 1);
   * console.log(queue.size); // 1
   * ```
   */
  get size(): number {
    return this.heap.length;
  }

  /**
   * Checks if the queue is empty.
   * @returns True if the queue is empty, false otherwise
   * @example
   * ```typescript
   * const queue = new PriorityQueue<string>();
   * console.log(queue.isEmpty); // true
   * queue.enqueue("item", 1);
   * console.log(queue.isEmpty); // false
   * ```
   */
  get isEmpty(): boolean {
    return this.heap.length === 0;
  }

  /**
   * Adds an element to the queue with a specified priority.
   * Higher priority values are dequeued first.
   * @param value - The value to add
   * @param priority - The priority value (higher values have higher priority)
   * @example
   * ```typescript
   * const queue = new PriorityQueue<string>();
   * queue.enqueue("low", 1);
   * queue.enqueue("high", 10);
   * queue.enqueue("medium", 5);
   * console.log(queue.dequeue()); // "high"
   * ```
   */
  enqueue(value: T, priority: number): void {
    this.heap.push({ value, priority });
    this.heapifyUp(this.heap.length - 1);
  }

  /**
   * Adds an element to the end of the queue with lowest priority.
   * This element will be dequeued last (FIFO for equal lowest priority).
   * @param value - The value to add
   * @example
   * ```typescript
   * const queue = new PriorityQueue<string>();
   * queue.enqueue("high", 10);
   * queue.enqueue("medium", 5);
   * queue.enqueueBack("back1");
   * queue.enqueueBack("back2");
   * console.log(queue.dequeue()); // "high"
   * console.log(queue.dequeue()); // "medium"
   * console.log(queue.dequeue()); // "back1"
   * console.log(queue.dequeue()); // "back2"
   * ```
   */
  enqueueBack(value: T): void {
    const lowestPriority = this.getLowestPriority();
    const newPriority = lowestPriority - 1;
    this.heap.push({ value, priority: newPriority });
    this.heapifyUp(this.heap.length - 1);
  }

  /**
   * Removes and returns the element with the highest priority.
   * @returns The element with highest priority, or undefined if queue is empty
   * @example
   * ```typescript
   * const queue = new PriorityQueue<string>();
   * queue.enqueue("low", 1);
   * queue.enqueue("high", 10);
   * console.log(queue.dequeue()); // "high"
   * console.log(queue.dequeue()); // "low"
   * console.log(queue.dequeue()); // undefined
   * ```
   */
  dequeue(): T | undefined {
    if (this.heap.length === 0) {
      return;
    }

    if (this.heap.length === 1) {
      return this.heap.pop()?.value;
    }

    const result = this.heap[0].value;
    // biome-ignore lint/style/noNonNullAssertion: pop() cannot return undefined when heap.length > 1
    this.heap[0] = this.heap.pop()!;
    this.heapifyDown(0);
    return result;
  }

  /**
   * Returns the element with the highest priority without removing it.
   * @returns The element with highest priority, or undefined if queue is empty
   * @example
   * ```typescript
   * const queue = new PriorityQueue<string>();
   * queue.enqueue("low", 1);
   * queue.enqueue("high", 10);
   * console.log(queue.peek()); // "high"
   * console.log(queue.size); // 2 (element not removed)
   * ```
   */
  peek(): T | undefined {
    return this.heap[0]?.value;
  }

  /**
   * Returns the priority of the element with the highest priority.
   * @returns The highest priority value, or undefined if queue is empty
   * @example
   * ```typescript
   * const queue = new PriorityQueue<string>();
   * queue.enqueue("low", 1);
   * queue.enqueue("high", 10);
   * console.log(queue.peekPriority()); // 10
   * ```
   */
  peekPriority(): number | undefined {
    return this.heap[0]?.priority;
  }

  /**
   * Removes all elements from the queue.
   * @example
   * ```typescript
   * const queue = new PriorityQueue<string>();
   * queue.enqueue("item1", 1);
   * queue.enqueue("item2", 2);
   * console.log(queue.size); // 2
   * queue.clear();
   * console.log(queue.size); // 0
   * console.log(queue.isEmpty); // true
   * ```
   */
  clear(): void {
    this.heap = [];
  }

  /**
   * Returns an array of all elements in the queue (without removing them).
   * The order is not guaranteed to be sorted by priority.
   * @returns Array of all elements in the queue
   * @example
   * ```typescript
   * const queue = new PriorityQueue<string>();
   * queue.enqueue("low", 1);
   * queue.enqueue("high", 10);
   * queue.enqueue("medium", 5);
   * console.log(queue.toArray()); // ["high", "medium", "low"] (order may vary)
   * ```
   */
  toArray(): T[] {
    return this.heap.map((item) => item.value);
  }

  /**
   * Returns an array of all elements with their priorities.
   * The order is not guaranteed to be sorted by priority.
   * @returns Array of all elements with their priorities
   * @example
   * ```typescript
   * const queue = new PriorityQueue<string>();
   * queue.enqueue("low", 1);
   * queue.enqueue("high", 10);
   * console.log(queue.toArrayWithPriorities()); 
   * // [{ value: "high", priority: 10 }, { value: "low", priority: 1 }] (order may vary)
   * ```
   */
  toArrayWithPriorities(): Array<{ value: T; priority: number }> {
    return [...this.heap];
  }

  /**
   * Gets the lowest priority value in the queue.
   * @returns The lowest priority value, or 0 if queue is empty
   */
  private getLowestPriority(): number {
    if (this.heap.length === 0) {
      return 0;
    }
    return Math.min(...this.heap.map(item => item.priority));
  }

  /**
   * Builds a max heap from the current heap array.
   */
  private buildHeap(): void {
    for (
      let index = Math.floor(this.heap.length / 2) - 1;
      index >= 0;
      index--
    ) {
      this.heapifyDown(index);
    }
  }

  /**
   * Moves an element up the heap to maintain heap property.
   * @param index - The index of the element to move up
   */
  private heapifyUp(index: number): void {
    let currentIndex = index;
    while (currentIndex > 0) {
      const parentIndex = Math.floor((currentIndex - 1) / 2);
      if (this.heap[currentIndex].priority <= this.heap[parentIndex].priority) {
        break;
      }
      this.swap(currentIndex, parentIndex);
      currentIndex = parentIndex;
    }
  }

  /**
   * Moves an element down the heap to maintain heap property.
   * @param index - The index of the element to move down
   */
  private heapifyDown(index: number): void {
    let currentIndex = index;
    while (true) {
      const leftChild = 2 * currentIndex + 1;
      const rightChild = 2 * currentIndex + 2;
      let largest = currentIndex;

      if (
        leftChild < this.heap.length &&
        this.heap[leftChild].priority > this.heap[largest].priority
      ) {
        largest = leftChild;
      }

      if (
        rightChild < this.heap.length &&
        this.heap[rightChild].priority > this.heap[largest].priority
      ) {
        largest = rightChild;
      }

      if (largest === currentIndex) {
        break;
      }

      this.swap(currentIndex, largest);
      currentIndex = largest;
    }
  }

  /**
   * Swaps two elements in the heap.
   * @param i - First index
   * @param j - Second index
   */
  private swap(index: number, index_: number): void {
    [this.heap[index], this.heap[index_]] = [
      this.heap[index_],
      this.heap[index],
    ];
  }
}
