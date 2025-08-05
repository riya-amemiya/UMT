import type { Graph, GraphOptions } from "$/graph";

/**
 * Creates a new empty graph with specified options
 * @param options - Graph configuration options
 * @returns A new empty graph
 * @example createGraph<string>({ directed: true }) // Creates a directed graph for string vertices
 */
export const createGraph = <T>(options: GraphOptions = {}): Graph<T> => {
  const { directed = false } = options;
  return {
    adjacencyList: new Map<T, Map<T, number>>(),
    directed,
  };
};
