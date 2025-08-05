import {
  createGraph,
  addEdge,
  removeEdge,
  bfs,
  dfs,
  dijkstra,
  aStar,
  topoSort,
  connectedComponents,
  hasCycle,
} from "@/Graph";

describe("Graph Integration Tests", () => {
  describe("Social Network Analysis", () => {
    it("should analyze friendship network connections", () => {
      let network = createGraph<string>({ directed: false });

      network = addEdge(network, "Alice", "Bob");
      network = addEdge(network, "Alice", "Charlie");
      network = addEdge(network, "Bob", "David");
      network = addEdge(network, "Charlie", "Eve");
      network = addEdge(network, "David", "Eve");

      const aliceFriends = bfs(network, "Alice");
      expect(aliceFriends).toHaveLength(5);
      expect(aliceFriends).toContain("Alice");
      expect(aliceFriends).toContain("Bob");
      expect(aliceFriends).toContain("Charlie");
      expect(aliceFriends).toContain("David");
      expect(aliceFriends).toContain("Eve");

      const friendGroups = connectedComponents(network);
      expect(friendGroups).toHaveLength(1);
      expect(friendGroups[0]).toHaveLength(5);

      expect(hasCycle(network)).toBe(true);
    });
  });

  describe("Task Dependency Management", () => {
    it("should manage project task dependencies", () => {
      let tasks = createGraph<string>({ directed: true });

      tasks = addEdge(tasks, "Planning", "Design");
      tasks = addEdge(tasks, "Design", "Frontend");
      tasks = addEdge(tasks, "Design", "Backend");
      tasks = addEdge(tasks, "Frontend", "Testing");
      tasks = addEdge(tasks, "Backend", "Testing");
      tasks = addEdge(tasks, "Testing", "Deployment");

      const executionOrder = topoSort(tasks);
      expect(executionOrder[0]).toBe("Planning");
      expect(executionOrder[executionOrder.length - 1]).toBe("Deployment");
      expect(executionOrder.indexOf("Design")).toBeLessThan(
        executionOrder.indexOf("Frontend"),
      );
      expect(executionOrder.indexOf("Design")).toBeLessThan(
        executionOrder.indexOf("Backend"),
      );
      expect(executionOrder.indexOf("Frontend")).toBeLessThan(
        executionOrder.indexOf("Testing"),
      );
      expect(executionOrder.indexOf("Backend")).toBeLessThan(
        executionOrder.indexOf("Testing"),
      );

      expect(hasCycle(tasks)).toBe(false);
    });

    it("should detect circular dependencies", () => {
      let tasks = createGraph<string>({ directed: true });

      tasks = addEdge(tasks, "A", "B");
      tasks = addEdge(tasks, "B", "C");
      tasks = addEdge(tasks, "C", "A");

      expect(hasCycle(tasks)).toBe(true);
      expect(() => topoSort(tasks)).toThrow("Graph contains a cycle");
    });
  });

  describe("Route Planning System", () => {
    it("should find optimal routes between cities", () => {
      let cities = createGraph<string>({ directed: true });

      cities = addEdge(cities, "Tokyo", "Osaka", 400);
      cities = addEdge(cities, "Tokyo", "Nagoya", 300);
      cities = addEdge(cities, "Nagoya", "Osaka", 150);
      cities = addEdge(cities, "Osaka", "Kyoto", 50);
      cities = addEdge(cities, "Tokyo", "Kyoto", 500);

      const shortestPath = dijkstra(cities, "Tokyo", "Kyoto");
      expect(shortestPath.path).toEqual(["Tokyo", "Osaka", "Kyoto"]);
      expect(shortestPath.cost).toBe(450);

      const heuristic = (from: string, to: string) => {
        const distances: Record<string, Record<string, number>> = {
          Tokyo: { Kyoto: 450 },
          Nagoya: { Kyoto: 200 },
          Osaka: { Kyoto: 50 },
          Kyoto: { Kyoto: 0 },
        };
        return distances[from]?.[to] ?? 0;
      };

      const aStarPath = aStar(cities, "Tokyo", "Kyoto", heuristic);
      expect(aStarPath.path).toEqual(shortestPath.path);
      expect(aStarPath.cost).toBe(shortestPath.cost);
    });
  });

  describe("Graph Modification Workflow", () => {
    it("should handle complex graph modifications", () => {
      let graph = createGraph<number>({ directed: false });

      graph = addEdge(graph, 1, 2, 10);
      graph = addEdge(graph, 2, 3, 20);
      graph = addEdge(graph, 3, 4, 30);
      graph = addEdge(graph, 1, 4, 100);

      expect(bfs(graph, 1)).toEqual([1, 2, 4, 3]);
      expect(hasCycle(graph)).toBe(true);

      graph = removeEdge(graph, 1, 4);
      expect(hasCycle(graph)).toBe(false);

      expect(bfs(graph, 1)).toEqual([1, 2, 3, 4]);
      expect(dfs(graph, 1)).toEqual([1, 2, 3, 4]);

      const components = connectedComponents(graph);
      expect(components).toHaveLength(1);
      expect(components[0]).toHaveLength(4);
    });
  });

  describe("Large Graph Performance", () => {
    it("should handle moderately large graphs efficiently", () => {
      let graph = createGraph<number>({ directed: false });

      for (let i = 0; i < 99; i++) {
        graph = addEdge(graph, i, i + 1);
      }

      graph = addEdge(graph, 0, 50);
      graph = addEdge(graph, 25, 75);

      const startTime = Date.now();

      const bfsResult = bfs(graph, 0);
      const dfsResult = dfs(graph, 0);
      const components = connectedComponents(graph);
      const cycleExists = hasCycle(graph);

      const endTime = Date.now();

      expect(bfsResult).toHaveLength(100);
      expect(dfsResult).toHaveLength(100);
      expect(components).toHaveLength(1);
      expect(cycleExists).toBe(true);

      expect(endTime - startTime).toBeLessThan(100);
    });
  });
});
