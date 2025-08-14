import type { Graph } from "$/graph";

/**
 * Adds a weighted edge between two vertices in the graph
 * @param graph - The input graph
 * @param from - The source vertex
 * @param to - The destination vertex
 * @param weight - The weight of the edge (default: 1)
 * @returns A new graph with the added edge
 * @example addEdge(graph, "A", "B", 5) // Adds edge A -> B with weight 5
 */
export const addEdge = <T>(
  graph: Graph<T>,
  from: T,
  to: T,
  weight = 1,
): Graph<T> => {
  const newGraph: Graph<T> = {
    adjacencyList: new Map(graph.adjacencyList),
    directed: graph.directed,
  };

  if (newGraph.adjacencyList.has(from)) {
    newGraph.adjacencyList.set(from, new Map(newGraph.adjacencyList.get(from)));
  } else {
    newGraph.adjacencyList.set(from, new Map());
  }

  if (!newGraph.adjacencyList.has(to)) {
    newGraph.adjacencyList.set(to, new Map());
  }

  newGraph.adjacencyList.get(from)?.set(to, weight);

  if (!newGraph.directed) {
    newGraph.adjacencyList.set(to, new Map(newGraph.adjacencyList.get(to)));
    newGraph.adjacencyList.get(to)?.set(from, weight);
  }

  return newGraph;
};
