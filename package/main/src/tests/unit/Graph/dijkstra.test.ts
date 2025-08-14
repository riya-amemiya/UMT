import { createGraph } from "@/Graph/createGraph";
import { addEdge } from "@/Graph/addEdge";
import { dijkstra } from "@/Graph/dijkstra";

describe("dijkstra", () => {
  it("should find shortest path", () => {
    let graph = createGraph<string>({ directed: true });
    graph = addEdge(graph, "A", "B", 4);
    graph = addEdge(graph, "A", "C", 2);
    graph = addEdge(graph, "C", "B", 1);
    graph = addEdge(graph, "B", "D", 3);
    graph = addEdge(graph, "C", "D", 5);

    const result = dijkstra(graph, "A", "D");

    expect(result.path).toEqual(["A", "C", "B", "D"]);
    expect(result.cost).toBe(6);
  });

  it("should return empty path when no path exists", () => {
    let graph = createGraph<string>({ directed: true });
    graph = addEdge(graph, "A", "B", 1);
    graph = addEdge(graph, "C", "D", 1);

    const result = dijkstra(graph, "A", "D");

    expect(result.path).toEqual([]);
    expect(result.cost).toBe(Number.POSITIVE_INFINITY);
  });

  it("should handle direct path", () => {
    let graph = createGraph<string>({ directed: true });
    graph = addEdge(graph, "A", "B", 5);

    const result = dijkstra(graph, "A", "B");

    expect(result.path).toEqual(["A", "B"]);
    expect(result.cost).toBe(5);
  });

  it("should handle same start and goal", () => {
    let graph = createGraph<string>({ directed: true });
    graph = addEdge(graph, "A", "B", 1);

    const result = dijkstra(graph, "A", "A");

    expect(result.path).toEqual(["A"]);
    expect(result.cost).toBe(0);
  });

  it("should handle graph with unreachable vertices", () => {
    let graph = createGraph<string>({ directed: true });
    graph = addEdge(graph, "A", "B", 1);
    // Add isolated vertex
    graph.adjacencyList.set("C", new Map());

    const result = dijkstra(graph, "A", "C");

    expect(result.path).toEqual([]);
    expect(result.cost).toBe(Number.POSITIVE_INFINITY);
  });

  it("should handle empty graph", () => {
    const graph = createGraph<string>({ directed: true });

    const result = dijkstra(graph, "A", "B");

    expect(result.path).toEqual([]);
    expect(result.cost).toBe(Number.POSITIVE_INFINITY);
  });
});
