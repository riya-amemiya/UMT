import { describe, expect, it } from "@jest/globals";
import { PriorityQueue } from "@/DataStructure/priorityQueue";

describe("PriorityQueue", () => {
  describe("constructor", () => {
    it("should create an empty queue", () => {
      const queue = new PriorityQueue<string>();
      expect(queue.size).toBe(0);
      expect(queue.isEmpty).toBe(true);
    });

    it("should create a queue with initial elements", () => {
      const initialElements = [
        { value: "low", priority: 1 },
        { value: "high", priority: 3 },
        { value: "medium", priority: 2 },
      ];
      const queue = new PriorityQueue<string>(initialElements);
      expect(queue.size).toBe(3);
      expect(queue.peek()).toBe("high");
    });
  });

  describe("enqueue", () => {
    it("should add elements to the queue", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("first", 1);
      queue.enqueue("second", 2);
      queue.enqueue("third", 3);

      expect(queue.size).toBe(3);
      expect(queue.isEmpty).toBe(false);
    });

    it("should maintain priority order", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("low", 1);
      queue.enqueue("high", 3);
      queue.enqueue("medium", 2);

      expect(queue.peek()).toBe("high");
    });
  });

  describe("enqueueBack", () => {
    it("should add elements to the back of the queue", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("high", 10);
      queue.enqueue("medium", 5);
      queue.enqueueBack("back1");
      queue.enqueueBack("back2");

      expect(queue.size).toBe(4);
      expect(queue.dequeue()).toBe("high");
      expect(queue.dequeue()).toBe("medium");
      expect(queue.dequeue()).toBe("back1");
      expect(queue.dequeue()).toBe("back2");
    });

    it("should work with empty queue", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueueBack("first");
      queue.enqueueBack("second");

      expect(queue.dequeue()).toBe("first");
      expect(queue.dequeue()).toBe("second");
    });

    it("should maintain FIFO order for back elements", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueueBack("first");
      queue.enqueueBack("second");
      queue.enqueueBack("third");

      expect(queue.dequeue()).toBe("first");
      expect(queue.dequeue()).toBe("second");
      expect(queue.dequeue()).toBe("third");
    });

    it("should place back elements after priority elements", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueueBack("back1");
      queue.enqueue("priority", 1);
      queue.enqueueBack("back2");

      expect(queue.dequeue()).toBe("priority");
      expect(queue.dequeue()).toBe("back1");
      expect(queue.dequeue()).toBe("back2");
    });
  });

  describe("dequeue", () => {
    it("should return undefined for empty queue", () => {
      const queue = new PriorityQueue<string>();
      expect(queue.dequeue()).toBeUndefined();
    });

    it("should remove and return highest priority element", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("low", 1);
      queue.enqueue("high", 3);
      queue.enqueue("medium", 2);

      expect(queue.dequeue()).toBe("high");
      expect(queue.dequeue()).toBe("medium");
      expect(queue.dequeue()).toBe("low");
      expect(queue.dequeue()).toBeUndefined();
    });

    it("should handle single element queue", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("single", 1);

      expect(queue.dequeue()).toBe("single");
      expect(queue.isEmpty).toBe(true);
    });

    it("should handle elements with same priority", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("first", 1);
      queue.enqueue("second", 1);
      queue.enqueue("third", 1);

      const results = [queue.dequeue(), queue.dequeue(), queue.dequeue()];
      expect(results).toContain("first");
      expect(results).toContain("second");
      expect(results).toContain("third");
    });
  });

  describe("peek", () => {
    it("should return undefined for empty queue", () => {
      const queue = new PriorityQueue<string>();
      expect(queue.peek()).toBeUndefined();
    });

    it("should return highest priority element without removing it", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("low", 1);
      queue.enqueue("high", 3);
      queue.enqueue("medium", 2);

      expect(queue.peek()).toBe("high");
      expect(queue.size).toBe(3);
      expect(queue.peek()).toBe("high");
    });
  });

  describe("peekPriority", () => {
    it("should return undefined for empty queue", () => {
      const queue = new PriorityQueue<string>();
      expect(queue.peekPriority()).toBeUndefined();
    });

    it("should return highest priority value", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("low", 1);
      queue.enqueue("high", 3);
      queue.enqueue("medium", 2);

      expect(queue.peekPriority()).toBe(3);
    });
  });

  describe("clear", () => {
    it("should remove all elements", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("first", 1);
      queue.enqueue("second", 2);
      queue.enqueue("third", 3);

      queue.clear();

      expect(queue.size).toBe(0);
      expect(queue.isEmpty).toBe(true);
      expect(queue.peek()).toBeUndefined();
    });
  });

  describe("toArray", () => {
    it("should return empty array for empty queue", () => {
      const queue = new PriorityQueue<string>();
      expect(queue.toArray()).toEqual([]);
    });

    it("should return all elements", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("first", 1);
      queue.enqueue("second", 2);
      queue.enqueue("third", 3);

      const array = queue.toArray();
      expect(array).toHaveLength(3);
      expect(array).toContain("first");
      expect(array).toContain("second");
      expect(array).toContain("third");
    });
  });

  describe("toArrayWithPriorities", () => {
    it("should return empty array for empty queue", () => {
      const queue = new PriorityQueue<string>();
      expect(queue.toArrayWithPriorities()).toEqual([]);
    });

    it("should return all elements with priorities", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("first", 1);
      queue.enqueue("second", 2);
      queue.enqueue("third", 3);

      const array = queue.toArrayWithPriorities();
      expect(array).toHaveLength(3);
      expect(array).toContainEqual({ value: "first", priority: 1 });
      expect(array).toContainEqual({ value: "second", priority: 2 });
      expect(array).toContainEqual({ value: "third", priority: 3 });
    });
  });

  describe("edge cases", () => {
    it("should handle negative priorities", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("negative", -5);
      queue.enqueue("zero", 0);
      queue.enqueue("positive", 5);

      expect(queue.dequeue()).toBe("positive");
      expect(queue.dequeue()).toBe("zero");
      expect(queue.dequeue()).toBe("negative");
    });

    it("should handle decimal priorities", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("low", 1.1);
      queue.enqueue("high", 1.9);
      queue.enqueue("medium", 1.5);

      expect(queue.dequeue()).toBe("high");
      expect(queue.dequeue()).toBe("medium");
      expect(queue.dequeue()).toBe("low");
    });

    it("should handle zero priority", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("zero", 0);
      queue.enqueue("positive", 1);
      queue.enqueue("negative", -1);

      expect(queue.dequeue()).toBe("positive");
      expect(queue.dequeue()).toBe("zero");
      expect(queue.dequeue()).toBe("negative");
    });

    it("should handle very large priorities", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("max", Number.MAX_SAFE_INTEGER);
      queue.enqueue("min", Number.MIN_SAFE_INTEGER);
      queue.enqueue("normal", 1);

      expect(queue.dequeue()).toBe("max");
      expect(queue.dequeue()).toBe("normal");
      expect(queue.dequeue()).toBe("min");
    });

    it("should handle Infinity priorities", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("positive-infinity", Number.POSITIVE_INFINITY);
      queue.enqueue("negative-infinity", Number.NEGATIVE_INFINITY);
      queue.enqueue("normal", 1);

      expect(queue.dequeue()).toBe("positive-infinity");
      expect(queue.dequeue()).toBe("normal");
      expect(queue.dequeue()).toBe("negative-infinity");
    });
  });

  describe("data type compatibility", () => {
    it("should work with different data types", () => {
      interface Task {
        id: string;
        description: string;
      }

      const queue = new PriorityQueue<Task>();

      queue.enqueue({ id: "1", description: "Low priority task" }, 1);
      queue.enqueue({ id: "2", description: "High priority task" }, 3);
      queue.enqueue({ id: "3", description: "Medium priority task" }, 2);

      const highPriorityTask = queue.dequeue();
      expect(highPriorityTask).toBeDefined();
      expect(highPriorityTask?.id).toBe("2");
      expect(highPriorityTask?.description).toBe("High priority task");
    });

    it("should work with numbers", () => {
      const queue = new PriorityQueue<number>();
      queue.enqueue(100, 1);
      queue.enqueue(300, 3);
      queue.enqueue(200, 2);

      expect(queue.dequeue()).toBe(300);
      expect(queue.dequeue()).toBe(200);
      expect(queue.dequeue()).toBe(100);
    });

    it("should work with arrays", () => {
      const queue = new PriorityQueue<number[]>();
      queue.enqueue([1, 2, 3], 1);
      queue.enqueue([4, 5, 6], 3);
      queue.enqueue([7, 8, 9], 2);

      expect(queue.dequeue()).toEqual([4, 5, 6]);
      expect(queue.dequeue()).toEqual([7, 8, 9]);
      expect(queue.dequeue()).toEqual([1, 2, 3]);
    });

    it("should work with null and undefined values", () => {
      const queue = new PriorityQueue<string | null | undefined>();
      queue.enqueue(null, 1);
      queue.enqueue(undefined, 3);
      queue.enqueue("string", 2);

      expect(queue.dequeue()).toBe(undefined);
      expect(queue.dequeue()).toBe("string");
      expect(queue.dequeue()).toBe(null);
    });
  });

  describe("complex scenarios", () => {
    it("should handle mixed operations", () => {
      const queue = new PriorityQueue<number>();

      queue.enqueue(10, 1);
      queue.enqueue(30, 3);
      queue.enqueue(20, 2);

      expect(queue.dequeue()).toBe(30);

      queue.enqueue(40, 4);
      queue.enqueue(15, 1.5);

      expect(queue.dequeue()).toBe(40);
      expect(queue.dequeue()).toBe(20);
      expect(queue.dequeue()).toBe(15);
      expect(queue.dequeue()).toBe(10);
      expect(queue.isEmpty).toBe(true);
    });

    it("should handle large number of elements", () => {
      const queue = new PriorityQueue<number>();
      const elements = Array.from({ length: 1000 }, (_, i) => ({
        value: i,
        priority: Math.random(),
      }));

      for (const element of elements) {
        queue.enqueue(element.value, element.priority);
      }

      expect(queue.size).toBe(1000);

      let previousPriority = Number.POSITIVE_INFINITY;
      while (!queue.isEmpty) {
        const currentPriority = queue.peekPriority();
        expect(currentPriority).toBeDefined();
        if (currentPriority !== undefined) {
          expect(currentPriority).toBeLessThanOrEqual(previousPriority);
          previousPriority = currentPriority;
        }
        queue.dequeue();
      }
    });

    it("should handle priority queue as task scheduler", () => {
      interface Task {
        name: string;
        priority: number;
        deadline: Date;
      }

      const queue = new PriorityQueue<Task>();
      const now = new Date();

      queue.enqueue({ name: "urgent", priority: 10, deadline: new Date(now.getTime() + 3_600_000) }, 10);
      queue.enqueue({ name: "normal", priority: 5, deadline: new Date(now.getTime() + 7_200_000) }, 5);
      queue.enqueue({ name: "low", priority: 1, deadline: new Date(now.getTime() + 10_800_000) }, 1);

      const firstTask = queue.dequeue();
      expect(firstTask?.name).toBe("urgent");
      expect(firstTask?.priority).toBe(10);
    });

    it("should handle dynamic priority updates via re-enqueue", () => {
      const queue = new PriorityQueue<string>();
      queue.enqueue("task1", 1);
      queue.enqueue("task2", 2);
      queue.enqueue("task3", 3);

      const task = queue.dequeue();
      expect(task).toBe("task3");
      
      queue.enqueue("task3", 0.5);
      
      expect(queue.dequeue()).toBe("task2");
      expect(queue.dequeue()).toBe("task1");
      expect(queue.dequeue()).toBe("task3");
    });
  });
});
