import { createGraph } from "@/Graph/createGraph";
import { addEdge } from "@/Graph/addEdge";
import { topoSort } from "@/Graph/topoSort";

describe("topoSort", () => {
  it("should perform topological sort on DAG", () => {
    let graph = createGraph<string>({ directed: true });
    graph = addEdge(graph, "A", "B");
    graph = addEdge(graph, "A", "C");
    graph = addEdge(graph, "B", "D");
    graph = addEdge(graph, "C", "D");

    const result = topoSort(graph);

    expect(result).toHaveLength(4);
    expect(result.indexOf("A")).toBeLessThan(result.indexOf("B"));
    expect(result.indexOf("A")).toBeLessThan(result.indexOf("C"));
    expect(result.indexOf("B")).toBeLessThan(result.indexOf("D"));
    expect(result.indexOf("C")).toBeLessThan(result.indexOf("D"));
  });

  it("should throw error for undirected graph", () => {
    const graph = createGraph<string>({ directed: false });

    expect(() => topoSort(graph)).toThrow(
      "Topological sort is only applicable to directed graphs",
    );
  });

  it("should throw error for cyclic graph", () => {
    let graph = createGraph<string>({ directed: true });
    graph = addEdge(graph, "A", "B");
    graph = addEdge(graph, "B", "C");
    graph = addEdge(graph, "C", "A");

    expect(() => topoSort(graph)).toThrow("Graph contains a cycle");
  });

  it("should handle single node", () => {
    let graph = createGraph<string>({ directed: true });
    graph = addEdge(graph, "A", "A");
    graph.adjacencyList.delete("A");
    graph.adjacencyList.set("A", new Map());

    const result = topoSort(graph);

    expect(result).toEqual(["A"]);
  });

  it("should handle empty graph", () => {
    const graph = createGraph<string>({ directed: true });

    const result = topoSort(graph);

    expect(result).toEqual([]);
  });
});
