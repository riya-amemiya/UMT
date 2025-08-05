import type { Graph } from "$/graph";

/**
 * Removes an edge between two vertices in the graph
 * @param graph - The input graph
 * @param from - The source vertex
 * @param to - The destination vertex
 * @returns A new graph with the removed edge
 * @example removeEdge(graph, "A", "B") // Removes edge A -> B
 */
export const removeEdge = <T>(graph: Graph<T>, from: T, to: T): Graph<T> => {
  const newGraph: Graph<T> = {
    adjacencyList: new Map(graph.adjacencyList),
    directed: graph.directed,
  };

  if (newGraph.adjacencyList.has(from)) {
    const neighbors = new Map(newGraph.adjacencyList.get(from));
    neighbors.delete(to);
    newGraph.adjacencyList.set(from, neighbors);
  }

  if (!newGraph.directed && newGraph.adjacencyList.has(to)) {
    const neighbors = new Map(newGraph.adjacencyList.get(to));
    neighbors.delete(from);
    newGraph.adjacencyList.set(to, neighbors);
  }

  return newGraph;
};
