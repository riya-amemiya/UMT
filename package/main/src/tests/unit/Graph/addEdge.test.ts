import { createGraph } from "@/Graph/createGraph";
import { addEdge } from "@/Graph/addEdge";

describe("addEdge", () => {
  it("should add an edge to undirected graph", () => {
    const graph = createGraph<string>();
    const newGraph = addEdge(graph, "A", "B", 5);

    expect(newGraph.adjacencyList.get("A")?.get("B")).toBe(5);
    expect(newGraph.adjacencyList.get("B")?.get("A")).toBe(5);
  });

  it("should add an edge to directed graph", () => {
    const graph = createGraph<string>({ directed: true });
    const newGraph = addEdge(graph, "A", "B", 3);

    expect(newGraph.adjacencyList.get("A")?.get("B")).toBe(3);
    expect(newGraph.adjacencyList.get("B")?.has("A")).toBe(false);
  });

  it("should use default weight of 1", () => {
    const graph = createGraph<string>();
    const newGraph = addEdge(graph, "A", "B");

    expect(newGraph.adjacencyList.get("A")?.get("B")).toBe(1);
    expect(newGraph.adjacencyList.get("B")?.get("A")).toBe(1);
  });

  it("should not mutate original graph", () => {
    const graph = createGraph<string>();
    const newGraph = addEdge(graph, "A", "B");

    expect(graph.adjacencyList.size).toBe(0);
    expect(newGraph.adjacencyList.size).toBe(2);
  });

  it("should handle adding edge between completely new nodes", () => {
    const graph = createGraph<string>({ directed: false });
    const newGraph = addEdge(graph, "X", "Y", 10);

    expect(newGraph.adjacencyList.get("X")?.get("Y")).toBe(10);
    expect(newGraph.adjacencyList.get("Y")?.get("X")).toBe(10);
    expect(newGraph.adjacencyList.size).toBe(2);
  });

  it("should handle adding edge from existing to new node in undirected graph", () => {
    let graph = createGraph<string>({ directed: false });
    graph = addEdge(graph, "A", "B");
    const newGraph = addEdge(graph, "A", "C", 5);

    expect(newGraph.adjacencyList.get("A")?.get("C")).toBe(5);
    expect(newGraph.adjacencyList.get("C")?.get("A")).toBe(5);
    expect(newGraph.adjacencyList.size).toBe(3);
  });

  it("should create new to-node in undirected graph when it doesn't exist", () => {
    const graph = createGraph<string>({ directed: false });
    // Pre-create the from node with some edges
    graph.adjacencyList.set("A", new Map([["X", 1]]));

    const newGraph = addEdge(graph, "A", "B", 3);

    expect(newGraph.adjacencyList.get("A")?.get("B")).toBe(3);
    expect(newGraph.adjacencyList.get("B")?.get("A")).toBe(3);
    expect(newGraph.adjacencyList.has("B")).toBe(true);
  });
});
