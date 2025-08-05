import type { Graph, PathResult } from "$/graph";

/**
 * Finds the shortest path between two vertices using Dijkstra's algorithm
 * @param graph - The input graph
 * @param start - The starting vertex
 * @param goal - The target vertex
 * @returns Path result containing the shortest path and its cost
 * @example dijkstra(graph, "A", "D") // Returns shortest path from A to D
 */
export const dijkstra = <T>(
  graph: Graph<T>,
  start: T,
  goal: T,
): PathResult<T> => {
  const distances = new Map<T, number>();
  const previous = new Map<T, T | null>();
  const unvisited = new Set<T>();

  for (const vertex of graph.adjacencyList.keys()) {
    distances.set(vertex, Number.POSITIVE_INFINITY);
    previous.set(vertex, null);
    unvisited.add(vertex);
  }
  distances.set(start, 0);

  while (unvisited.size > 0) {
    let current: T | null = null;
    let minDistance = Number.POSITIVE_INFINITY;

    for (const vertex of unvisited) {
      const distance = distances.get(vertex) ?? Number.POSITIVE_INFINITY;
      if (distance < minDistance) {
        minDistance = distance;
        current = vertex;
      }
    }

    if (current === null || minDistance === Number.POSITIVE_INFINITY) {
      break;
    }

    if (current === goal) {
      const path: T[] = [];
      let node: T | null = goal;

      while (node !== null) {
        path.unshift(node);
        node = previous.get(node) ?? null;
      }

      return {
        path,
        cost: distances.get(goal) ?? 0,
      };
    }

    unvisited.delete(current);

    const neighbors = graph.adjacencyList.get(current);
    if (neighbors) {
      for (const [neighbor, weight] of neighbors) {
        if (unvisited.has(neighbor)) {
          const alt = (distances.get(current) ?? 0) + weight;
          if (alt < (distances.get(neighbor) ?? Number.POSITIVE_INFINITY)) {
            distances.set(neighbor, alt);
            previous.set(neighbor, current);
          }
        }
      }
    }
  }

  return {
    path: [],
    cost: Number.POSITIVE_INFINITY,
  };
};
