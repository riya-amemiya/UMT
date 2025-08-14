import type { Graph, GraphTraversalOptions } from "$/graph";

/**
 * Performs depth-first search traversal on the graph
 * @param graph - The input graph
 * @param start - The starting vertex
 * @param options - Traversal options including visit callback and max depth
 * @returns Array of vertices in DFS order
 * @example dfs(graph, "A") // Returns vertices reachable from A in DFS order
 */
export const dfs = <T>(
  graph: Graph<T>,
  start: T,
  options: GraphTraversalOptions<T> = {},
): T[] => {
  const { onVisit, maxDepth = Number.POSITIVE_INFINITY } = options;
  const visited = new Set<T>();
  const result: T[] = [];

  const dfsHelper = (node: T, depth: number): void => {
    if (visited.has(node) || depth > maxDepth) {
      return;
    }

    visited.add(node);
    result.push(node);
    onVisit?.(node);

    const neighbors = graph.adjacencyList.get(node);
    if (neighbors) {
      for (const [neighbor] of neighbors) {
        if (!visited.has(neighbor)) {
          dfsHelper(neighbor, depth + 1);
        }
      }
    }
  };

  dfsHelper(start, 0);
  return result;
};
