import { createGraph } from "@/Graph/createGraph";
import { addEdge } from "@/Graph/addEdge";
import { connectedComponents } from "@/Graph/connectedComponents";

describe("connectedComponents", () => {
  it("should find connected components in undirected graph", () => {
    let graph = createGraph<string>({ directed: false });
    graph = addEdge(graph, "A", "B");
    graph = addEdge(graph, "B", "C");
    graph = addEdge(graph, "D", "E");

    const result = connectedComponents(graph);

    expect(result).toHaveLength(2);
    expect(
      result.some(
        (component) =>
          component.includes("A") &&
          component.includes("B") &&
          component.includes("C"),
      ),
    ).toBe(true);
    expect(
      result.some(
        (component) => component.includes("D") && component.includes("E"),
      ),
    ).toBe(true);
  });

  it("should throw error for directed graph", () => {
    const graph = createGraph<string>({ directed: true });

    expect(() => connectedComponents(graph)).toThrow(
      "Connected components is only applicable to undirected graphs",
    );
  });

  it("should handle single component", () => {
    let graph = createGraph<string>({ directed: false });
    graph = addEdge(graph, "A", "B");
    graph = addEdge(graph, "B", "C");
    graph = addEdge(graph, "C", "A");

    const result = connectedComponents(graph);

    expect(result).toHaveLength(1);
    expect(result[0]).toContain("A");
    expect(result[0]).toContain("B");
    expect(result[0]).toContain("C");
  });

  it("should handle isolated nodes", () => {
    const graph = createGraph<string>({ directed: false });
    graph.adjacencyList.set("A", new Map());
    graph.adjacencyList.set("B", new Map());
    graph.adjacencyList.set("C", new Map());

    const result = connectedComponents(graph);

    expect(result).toHaveLength(3);
    expect(result.every((component) => component.length === 1)).toBe(true);
  });

  it("should handle empty graph", () => {
    const graph = createGraph<string>({ directed: false });

    const result = connectedComponents(graph);

    expect(result).toEqual([]);
  });
});
