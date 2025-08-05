import { dfs } from "./dfs";

import type { Graph } from "$/graph";

/**
 * Finds all connected components in an undirected graph
 * @param graph - The input undirected graph
 * @returns Array of connected components, each containing vertices
 * @throws Error if graph is directed
 * @example connectedComponents(graph) // Returns [[A, B], [C, D]] for two components
 */
export const connectedComponents = <T>(graph: Graph<T>): T[][] => {
  if (graph.directed) {
    throw new Error(
      "Connected components is only applicable to undirected graphs",
    );
  }

  const visited = new Set<T>();
  const components: T[][] = [];

  for (const vertex of graph.adjacencyList.keys()) {
    if (!visited.has(vertex)) {
      const component = dfs(graph, vertex);
      components.push(component);

      for (const node of component) {
        visited.add(node);
      }
    }
  }

  return components;
};
