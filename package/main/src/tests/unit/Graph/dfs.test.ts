import { createGraph } from "@/Graph/createGraph";
import { addEdge } from "@/Graph/addEdge";
import { dfs } from "@/Graph/dfs";

describe("dfs", () => {
  it("should perform depth-first search correctly", () => {
    let graph = createGraph<string>();
    graph = addEdge(graph, "A", "B");
    graph = addEdge(graph, "A", "C");
    graph = addEdge(graph, "B", "D");
    graph = addEdge(graph, "C", "E");

    const result = dfs(graph, "A");

    expect(result).toContain("A");
    expect(result).toContain("B");
    expect(result).toContain("C");
    expect(result).toContain("D");
    expect(result).toContain("E");
    expect(result.length).toBe(5);
    expect(result[0]).toBe("A");
  });

  it("should handle single node", () => {
    let graph = createGraph<string>();
    graph = addEdge(graph, "A", "A");

    const result = dfs(graph, "A");

    expect(result).toEqual(["A"]);
  });

  it("should respect maxDepth option", () => {
    let graph = createGraph<string>();
    graph = addEdge(graph, "A", "B");
    graph = addEdge(graph, "B", "C");
    graph = addEdge(graph, "C", "D");

    const result = dfs(graph, "A", { maxDepth: 1 });

    expect(result).toEqual(["A", "B"]);
  });

  it("should call onVisit callback", () => {
    let graph = createGraph<string>();
    graph = addEdge(graph, "A", "B");

    const visited: string[] = [];
    dfs(graph, "A", { onVisit: (node) => visited.push(node) });

    expect(visited).toEqual(["A", "B"]);
  });
});
