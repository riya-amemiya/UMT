import { createGraph } from "@/Graph/createGraph";
import { addEdge } from "@/Graph/addEdge";
import { bfs } from "@/Graph/bfs";

describe("bfs", () => {
  it("should perform breadth-first search correctly", () => {
    let graph = createGraph<string>();
    graph = addEdge(graph, "A", "B");
    graph = addEdge(graph, "A", "C");
    graph = addEdge(graph, "B", "D");
    graph = addEdge(graph, "C", "E");

    const result = bfs(graph, "A");

    expect(result).toEqual(["A", "B", "C", "D", "E"]);
  });

  it("should handle single node", () => {
    let graph = createGraph<string>();
    graph = addEdge(graph, "A", "A");

    const result = bfs(graph, "A");

    expect(result).toEqual(["A"]);
  });

  it("should respect maxDepth option", () => {
    let graph = createGraph<string>();
    graph = addEdge(graph, "A", "B");
    graph = addEdge(graph, "B", "C");
    graph = addEdge(graph, "C", "D");

    const result = bfs(graph, "A", { maxDepth: 1 });

    expect(result).toEqual(["A", "B"]);
  });

  it("should call onVisit callback", () => {
    let graph = createGraph<string>();
    graph = addEdge(graph, "A", "B");

    const visited: string[] = [];
    bfs(graph, "A", { onVisit: (node) => visited.push(node) });

    expect(visited).toEqual(["A", "B"]);
  });
});
