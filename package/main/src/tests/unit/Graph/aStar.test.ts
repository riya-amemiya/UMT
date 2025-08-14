import { createGraph } from "@/Graph/createGraph";
import { addEdge } from "@/Graph/addEdge";
import { aStar } from "@/Graph/aStar";

describe("aStar", () => {
  const manhattanHeuristic = (a: string, b: string): number => {
    return Math.abs(a.charCodeAt(0) - b.charCodeAt(0));
  };

  it("should find shortest path with heuristic", () => {
    let graph = createGraph<string>({ directed: true });
    graph = addEdge(graph, "A", "B", 4);
    graph = addEdge(graph, "A", "C", 2);
    graph = addEdge(graph, "C", "B", 1);
    graph = addEdge(graph, "B", "D", 3);
    graph = addEdge(graph, "C", "D", 5);

    const result = aStar(graph, "A", "D", manhattanHeuristic);

    expect(result.path).toEqual(["A", "C", "B", "D"]);
    expect(result.cost).toBe(6);
  });

  it("should return empty path when no path exists", () => {
    let graph = createGraph<string>({ directed: true });
    graph = addEdge(graph, "A", "B", 1);
    graph = addEdge(graph, "C", "D", 1);

    const result = aStar(graph, "A", "D", manhattanHeuristic);

    expect(result.path).toEqual([]);
    expect(result.cost).toBe(Number.POSITIVE_INFINITY);
  });

  it("should handle direct path", () => {
    let graph = createGraph<string>({ directed: true });
    graph = addEdge(graph, "A", "B", 5);

    const result = aStar(graph, "A", "B", manhattanHeuristic);

    expect(result.path).toEqual(["A", "B"]);
    expect(result.cost).toBe(5);
  });

  it("should handle same start and goal", () => {
    let graph = createGraph<string>({ directed: true });
    graph = addEdge(graph, "A", "B", 1);

    const result = aStar(graph, "A", "A", manhattanHeuristic);

    expect(result.path).toEqual(["A"]);
    expect(result.cost).toBe(0);
  });

  it("should handle unreachable nodes in openSet", () => {
    const graph = createGraph<string>({ directed: true });
    graph.adjacencyList.set("A", new Map());
    graph.adjacencyList.set("B", new Map());

    const result = aStar(graph, "A", "B", manhattanHeuristic);

    expect(result.path).toEqual([]);
    expect(result.cost).toBe(Number.POSITIVE_INFINITY);
  });

  it("should handle infinite heuristic function", () => {
    const graph = createGraph<string>({ directed: true });
    graph.adjacencyList.set("A", new Map());
    graph.adjacencyList.set("B", new Map());

    const infiniteHeuristic = () => Number.POSITIVE_INFINITY;
    const result = aStar(graph, "A", "B", infiniteHeuristic);

    expect(result.path).toEqual([]);
    expect(result.cost).toBe(Number.POSITIVE_INFINITY);
  });
});
