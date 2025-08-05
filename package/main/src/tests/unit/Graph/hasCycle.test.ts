import { createGraph } from "@/Graph/createGraph";
import { addEdge } from "@/Graph/addEdge";
import { hasCycle } from "@/Graph/hasCycle";

describe("hasCycle", () => {
  describe("directed graph", () => {
    it("should detect cycle in directed graph", () => {
      let graph = createGraph<string>({ directed: true });
      graph = addEdge(graph, "A", "B");
      graph = addEdge(graph, "B", "C");
      graph = addEdge(graph, "C", "A");

      const result = hasCycle(graph);

      expect(result).toBe(true);
    });

    it("should not detect cycle in DAG", () => {
      let graph = createGraph<string>({ directed: true });
      graph = addEdge(graph, "A", "B");
      graph = addEdge(graph, "A", "C");
      graph = addEdge(graph, "B", "D");
      graph = addEdge(graph, "C", "D");

      const result = hasCycle(graph);

      expect(result).toBe(false);
    });

    it("should handle self-loop", () => {
      let graph = createGraph<string>({ directed: true });
      graph = addEdge(graph, "A", "A");

      const result = hasCycle(graph);

      expect(result).toBe(true);
    });
  });

  describe("undirected graph", () => {
    it("should detect cycle in undirected graph", () => {
      let graph = createGraph<string>({ directed: false });
      graph = addEdge(graph, "A", "B");
      graph = addEdge(graph, "B", "C");
      graph = addEdge(graph, "C", "A");

      const result = hasCycle(graph);

      expect(result).toBe(true);
    });

    it("should not detect cycle in tree", () => {
      let graph = createGraph<string>({ directed: false });
      graph = addEdge(graph, "A", "B");
      graph = addEdge(graph, "A", "C");
      graph = addEdge(graph, "B", "D");

      const result = hasCycle(graph);

      expect(result).toBe(false);
    });

    it("should handle self-loop", () => {
      let graph = createGraph<string>({ directed: false });
      graph = addEdge(graph, "A", "A");

      const result = hasCycle(graph);

      expect(result).toBe(true);
    });
  });

  it("should handle empty graph", () => {
    const directedGraph = createGraph<string>({ directed: true });
    const undirectedGraph = createGraph<string>({ directed: false });

    expect(hasCycle(directedGraph)).toBe(false);
    expect(hasCycle(undirectedGraph)).toBe(false);
  });
});
