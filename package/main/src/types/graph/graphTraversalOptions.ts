export interface GraphTraversalOptions<T> {
  onVisit?: (node: T) => void;
  maxDepth?: number;
}
