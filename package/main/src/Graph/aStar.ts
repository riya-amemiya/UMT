import type { Graph, HeuristicFunction, PathResult } from "$/graph";

/**
 * Finds the shortest path between two vertices using A* algorithm
 * @param graph - The input graph
 * @param start - The starting vertex
 * @param goal - The target vertex
 * @param heuristic - Heuristic function to estimate cost to goal
 * @returns Path result containing the shortest path and its cost
 * @example aStar(graph, "A", "D", (from, to) => manhattanDistance(from, to))
 */
export const aStar = <T>(
  graph: Graph<T>,
  start: T,
  goal: T,
  heuristic: HeuristicFunction<T>,
): PathResult<T> => {
  const openSet = new Set<T>([start]);
  const cameFrom = new Map<T, T>();

  const gScore = new Map<T, number>();
  const fScore = new Map<T, number>();

  for (const vertex of graph.adjacencyList.keys()) {
    gScore.set(vertex, Number.POSITIVE_INFINITY);
    fScore.set(vertex, Number.POSITIVE_INFINITY);
  }

  gScore.set(start, 0);
  fScore.set(start, heuristic(start, goal));

  while (openSet.size > 0) {
    let current: T | null = null;
    let lowestFScore = Number.POSITIVE_INFINITY;

    for (const node of openSet) {
      const score = fScore.get(node) ?? Number.POSITIVE_INFINITY;
      if (score < lowestFScore) {
        lowestFScore = score;
        current = node;
      }
    }

    if (current === null) {
      break;
    }

    if (current === goal) {
      const path: T[] = [];
      let node: T | undefined = goal;

      while (node !== undefined) {
        path.unshift(node);
        node = cameFrom.get(node);
      }

      return {
        path,
        cost: gScore.get(goal) ?? 0,
      };
    }

    openSet.delete(current);

    const neighbors = graph.adjacencyList.get(current);
    if (neighbors) {
      for (const [neighbor, weight] of neighbors) {
        const tentativeGScore = (gScore.get(current) ?? 0) + weight;

        if (
          tentativeGScore < (gScore.get(neighbor) ?? Number.POSITIVE_INFINITY)
        ) {
          cameFrom.set(neighbor, current);
          gScore.set(neighbor, tentativeGScore);
          fScore.set(neighbor, tentativeGScore + heuristic(neighbor, goal));
          openSet.add(neighbor);
        }
      }
    }
  }

  return {
    path: [],
    cost: Number.POSITIVE_INFINITY,
  };
};
