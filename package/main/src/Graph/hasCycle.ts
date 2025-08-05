import type { Graph } from "$/graph";

/**
 * Detects if the graph contains a cycle
 * @param graph - The input graph (directed or undirected)
 * @returns True if the graph contains a cycle, false otherwise
 * @example hasCycle(graph) // Returns true if graph has cycles
 */
export const hasCycle = <T>(graph: Graph<T>): boolean => {
  if (graph.directed) {
    const colors = new Map<T, "white" | "gray" | "black">();

    for (const vertex of graph.adjacencyList.keys()) {
      colors.set(vertex, "white");
    }

    const hasCycleDFS = (vertex: T): boolean => {
      colors.set(vertex, "gray");

      const neighbors = graph.adjacencyList.get(vertex);
      if (neighbors) {
        for (const [neighbor] of neighbors) {
          const color = colors.get(neighbor);
          if (color === "gray") {
            return true;
          }
          if (color === "white" && hasCycleDFS(neighbor)) {
            return true;
          }
        }
      }

      colors.set(vertex, "black");
      return false;
    };

    for (const vertex of graph.adjacencyList.keys()) {
      if (colors.get(vertex) === "white" && hasCycleDFS(vertex)) {
        return true;
      }
    }

    return false;
  }
  const visited = new Set<T>();

  const hasCycleDFS = (vertex: T, parent: T | null): boolean => {
    visited.add(vertex);

    const neighbors = graph.adjacencyList.get(vertex);
    if (neighbors) {
      for (const [neighbor] of neighbors) {
        if (!visited.has(neighbor)) {
          if (hasCycleDFS(neighbor, vertex)) {
            return true;
          }
        } else if (neighbor !== parent) {
          return true;
        }
      }
    }

    return false;
  };

  for (const vertex of graph.adjacencyList.keys()) {
    if (!visited.has(vertex) && hasCycleDFS(vertex, null)) {
      return true;
    }
  }

  return false;
};
