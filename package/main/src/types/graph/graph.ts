export interface Graph<T> {
  adjacencyList: Map<T, Map<T, number>>;
  directed: boolean;
}
