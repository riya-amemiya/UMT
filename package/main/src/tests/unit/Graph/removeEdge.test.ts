import { createGraph } from "@/Graph/createGraph";
import { addEdge } from "@/Graph/addEdge";
import { removeEdge } from "@/Graph/removeEdge";

describe("removeEdge", () => {
  it("should remove edge from undirected graph", () => {
    let graph = createGraph<string>();
    graph = addEdge(graph, "A", "B", 5);
    const newGraph = removeEdge(graph, "A", "B");

    expect(newGraph.adjacencyList.get("A")?.has("B")).toBe(false);
    expect(newGraph.adjacencyList.get("B")?.has("A")).toBe(false);
  });

  it("should remove edge from directed graph", () => {
    let graph = createGraph<string>({ directed: true });
    graph = addEdge(graph, "A", "B", 3);
    const newGraph = removeEdge(graph, "A", "B");

    expect(newGraph.adjacencyList.get("A")?.has("B")).toBe(false);
    expect(newGraph.adjacencyList.get("B")?.has("A")).toBe(false);
  });

  it("should handle non-existent edge", () => {
    const graph = createGraph<string>();
    const newGraph = removeEdge(graph, "A", "B");

    expect(newGraph.adjacencyList.size).toBe(0);
  });

  it("should not mutate original graph", () => {
    let graph = createGraph<string>();
    graph = addEdge(graph, "A", "B");
    const newGraph = removeEdge(graph, "A", "B");

    expect(graph.adjacencyList.get("A")?.has("B")).toBe(true);
    expect(newGraph.adjacencyList.get("A")?.has("B")).toBe(false);
  });

  it("should keep vertex even when no edges remain", () => {
    let graph = createGraph<string>();
    graph = addEdge(graph, "A", "B");
    const newGraph = removeEdge(graph, "A", "B");

    expect(newGraph.adjacencyList.has("A")).toBe(true);
    expect(newGraph.adjacencyList.has("B")).toBe(true);
    expect(newGraph.adjacencyList.get("A")?.size).toBe(0);
    expect(newGraph.adjacencyList.get("B")?.size).toBe(0);
  });
});
