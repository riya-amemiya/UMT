import type { Graph } from "$/graph";

/**
 * Performs topological sorting on a directed acyclic graph (DAG)
 * @param graph - The input directed graph
 * @returns Array of vertices in topological order
 * @throws Error if graph contains a cycle or is undirected
 * @example topoSort(dagGraph) // Returns vertices in dependency order
 */
export const topoSort = <T>(graph: Graph<T>): T[] => {
  if (!graph.directed) {
    throw new Error("Topological sort is only applicable to directed graphs");
  }

  const inDegree = new Map<T, number>();
  const queue: T[] = [];
  const result: T[] = [];

  for (const vertex of graph.adjacencyList.keys()) {
    inDegree.set(vertex, 0);
  }

  for (const [, neighbors] of graph.adjacencyList) {
    for (const [neighbor] of neighbors) {
      inDegree.set(neighbor, (inDegree.get(neighbor) ?? 0) + 1);
    }
  }

  for (const [vertex, degree] of inDegree) {
    if (degree === 0) {
      queue.push(vertex);
    }
  }

  while (queue.length > 0) {
    // biome-ignore lint/style/noNonNullAssertion: queue.length > 0 ensures item is not null
    const current = queue.shift()!;
    result.push(current);

    const neighbors = graph.adjacencyList.get(current);
    if (neighbors) {
      for (const [neighbor] of neighbors) {
        const newDegree = (inDegree.get(neighbor) ?? 0) - 1;
        inDegree.set(neighbor, newDegree);

        if (newDegree === 0) {
          queue.push(neighbor);
        }
      }
    }
  }

  if (result.length !== graph.adjacencyList.size) {
    throw new Error("Graph contains a cycle");
  }

  return result;
};
