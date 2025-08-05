import { createGraph } from "@/Graph/createGraph";

describe("createGraph", () => {
  it("should create an undirected graph by default", () => {
    const graph = createGraph<string>();

    expect(graph.directed).toBe(false);
    expect(graph.adjacencyList.size).toBe(0);
  });

  it("should create a directed graph when specified", () => {
    const graph = createGraph<string>({ directed: true });

    expect(graph.directed).toBe(true);
    expect(graph.adjacencyList.size).toBe(0);
  });

  it("should create an undirected graph explicitly", () => {
    const graph = createGraph<string>({ directed: false });

    expect(graph.directed).toBe(false);
    expect(graph.adjacencyList.size).toBe(0);
  });
});
