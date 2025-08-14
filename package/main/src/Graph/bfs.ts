import type { Graph, GraphTraversalOptions } from "$/graph";

/**
 * Performs breadth-first search traversal on the graph
 * @param graph - The input graph
 * @param start - The starting vertex
 * @param options - Traversal options including visit callback and max depth
 * @returns Array of vertices in BFS order
 * @example bfs(graph, "A") // Returns vertices reachable from A in BFS order
 */
export const bfs = <T>(
  graph: Graph<T>,
  start: T,
  options: GraphTraversalOptions<T> = {},
): T[] => {
  const { onVisit, maxDepth = Number.POSITIVE_INFINITY } = options;
  const visited = new Set<T>();
  const queue: Array<{ node: T; depth: number }> = [{ node: start, depth: 0 }];
  const result: T[] = [];

  while (queue.length > 0) {
    // biome-ignore lint/style/noNonNullAssertion: queue.length > 0 ensures item is not null
    const { node, depth } = queue.shift()!;

    if (visited.has(node) || depth > maxDepth) {
      continue;
    }

    visited.add(node);
    result.push(node);
    onVisit?.(node);

    const neighbors = graph.adjacencyList.get(node);
    if (neighbors) {
      for (const [neighbor] of neighbors) {
        if (!visited.has(neighbor)) {
          queue.push({ node: neighbor, depth: depth + 1 });
        }
      }
    }
  }

  return result;
};
