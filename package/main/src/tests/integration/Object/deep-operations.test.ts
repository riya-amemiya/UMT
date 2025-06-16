import { mergeDeep } from "@/Object/mergeDeep";
import { pickDeep } from "@/Object/pickDeep";
import { omit } from "@/Object/omit";
import { pick } from "@/Object/pick";
import { keyBy } from "@/Object/keyBy";
import { has } from "@/Object/has";

/**
 * Integration tests for Object manipulation functions
 *
 * Tests the interaction between deep object operations:
 * - Deep merging with selective picking
 * - Complex object transformations
 * - Nested property manipulations
 */
describe("Integration test for deep object operations", () => {
  it("should merge deep objects and pick specific nested properties", () => {
    const baseConfig = {
      app: {
        name: "MyApp",
        version: "1.0.0",
        features: {
          auth: true,
          logging: false,
        },
      },
      database: {
        host: "localhost",
        port: 5432,
      },
    };

    const envConfig = {
      app: {
        version: "1.2.0",
        features: {
          logging: true,
          analytics: true,
        },
      },
      database: {
        host: "prod-db.example.com",
      },
    };

    const merged = mergeDeep(baseConfig, envConfig);
    const essential = pickDeep(
      merged,
      "app.name",
      "app.version",
      "database.host",
    );

    expect(merged.app.features.auth).toBe(true);
    expect(merged.app.features.logging).toBe(true);
    expect(merged.app.features.analytics).toBe(true);

    expect(essential).toEqual({
      app: {
        name: "MyApp",
        version: "1.2.0",
      },
      database: {
        host: "prod-db.example.com",
      },
    });
  });

  it("should transform nested data structures with multiple operations", () => {
    const userData = [
      {
        id: 1,
        profile: {
          name: "Alice",
          email: "alice@example.com",
          settings: { theme: "dark", notifications: true },
        },
        metadata: { created: "2023-01-01", lastLogin: "2025-01-01" },
      },
      {
        id: 2,
        profile: {
          name: "Bob",
          email: "bob@example.com",
          settings: { theme: "light", notifications: false },
        },
        metadata: { created: "2023-02-01", lastLogin: "2025-01-02" },
      },
    ];

    const userIndex = keyBy(userData, "id");

    const userProfiles = Object.values(userIndex).map((user) =>
      pick(user, "id", "profile"),
    );

    const profilesWithoutEmail = userProfiles.map((user) => ({
      ...user,
      profile: omit(user.profile, "email"),
    }));

    expect(userIndex[1].profile.name).toBe("Alice");
    expect(profilesWithoutEmail[0]).toEqual({
      id: 1,
      profile: {
        name: "Alice",
        settings: { theme: "dark", notifications: true },
      },
    });
  });

  it("should handle complex nested merging with property checking", () => {
    const defaultConfig = {
      server: {
        port: 3000,
        middleware: {
          cors: { enabled: true, origins: ["http://localhost"] },
          auth: { enabled: false },
        },
      },
      features: {
        caching: true,
        logging: {
          level: "info",
          transports: ["console"],
        },
      },
    };

    const customConfig = {
      server: {
        port: 8080,
        middleware: {
          cors: { origins: ["https://example.com"] },
          rateLimit: { enabled: true, max: 100 },
        },
      },
      features: {
        logging: {
          level: "debug",
          transports: ["console", "file"],
        },
      },
    };

    const finalConfig = mergeDeep(defaultConfig, customConfig);

    expect(has(finalConfig, "server.middleware.cors.enabled")).toBe(true);
    expect(has(finalConfig, "server.middleware.rateLimit.enabled")).toBe(true);
    expect(has(finalConfig, "features.logging.transports")).toBe(true);

    expect(finalConfig.server.middleware.cors.enabled).toBe(true);
    expect(finalConfig.server.middleware.cors.origins).toEqual([
      "https://example.com",
    ]);
    expect(finalConfig.features.logging.transports).toEqual([
      "console",
      "file",
    ]);
  });

  it("should create filtered views of complex objects", () => {
    const productData = {
      id: "prod-123",
      details: {
        name: "Premium Widget",
        description: "High-quality widget",
        specs: {
          weight: "2kg",
          dimensions: { width: 10, height: 5, depth: 3 },
          materials: ["aluminum", "plastic"],
        },
      },
      pricing: {
        base: 99.99,
        currency: "USD",
        discounts: {
          bulk: 0.1,
          seasonal: 0.05,
        },
      },
      inventory: {
        stock: 50,
        warehouse: "WH-001",
      },
    };

    const publicView = pickDeep(
      productData,
      "id",
      "details.name",
      "details.specs.dimensions",
      "pricing.base",
      "pricing.currency",
    );

    const internalView = omit(productData, "pricing");
    const pricingOnly = pick(productData, "id", "pricing");

    expect(publicView).toEqual({
      id: "prod-123",
      details: {
        name: "Premium Widget",
        specs: {
          dimensions: { width: 10, height: 5, depth: 3 },
        },
      },
      pricing: {
        base: 99.99,
        currency: "USD",
      },
    });

    expect(has(internalView, "pricing")).toBe(false);
    expect(has(pricingOnly, "pricing.discounts.bulk")).toBe(true);
  });

  it("should handle deep merging with array data and selective extraction", () => {
    const baseData = {
      users: [
        { id: 1, name: "Alice", roles: ["user"] },
        { id: 2, name: "Bob", roles: ["user", "editor"] },
      ],
      permissions: {
        user: ["read"],
        editor: ["read", "write"],
        admin: ["read", "write", "delete"],
      },
    };

    const updateData = {
      users: [{ id: 3, name: "Charlie", roles: ["admin"] }],
      permissions: {
        editor: ["read", "write", "publish"],
        moderator: ["read", "moderate"],
      },
    };

    const allUsers = [...baseData.users, ...updateData.users];
    const mergedPermissions = mergeDeep(
      baseData.permissions,
      updateData.permissions,
    );
    const merged = { users: allUsers, permissions: mergedPermissions };

    const usersByRole = keyBy(merged.users, (user) => user.roles[0]);

    const adminPermissions = pick(merged.permissions, "admin");
    const nonUserPermissions = omit(merged.permissions, "user");

    expect(merged.users.length).toBe(3);
    expect(usersByRole["admin"].name).toBe("Charlie");
    expect(adminPermissions["admin"]).toEqual(["read", "write", "delete"]);
    expect(has(nonUserPermissions, "user")).toBe(false);
    expect(has(nonUserPermissions, "moderator")).toBe(true);
  });

  it("should chain multiple object operations in complex workflows", () => {
    interface User {
      id: string;
      profile: { name: string; email: string };
      settings: { theme: string; lang: string };
      metadata: { created: string; active: boolean };
    }

    interface ProcessedUser {
      id?: string;
      profile?: { name?: string; email?: string };
      settings?: { theme?: string; lang?: string };
      metadata?: { created?: string; active?: boolean };
      summary: {
        displayName: string;
        isActive: boolean;
      };
    }

    const apiResponse = {
      status: "success",
      data: {
        users: [
          {
            id: "user-1",
            profile: { name: "Alice", email: "alice@example.com" },
            settings: { theme: "dark", lang: "en" },
            metadata: { created: "2023-01-01", active: true },
          },
          {
            id: "user-2",
            profile: { name: "Bob", email: "bob@example.com" },
            settings: { theme: "light", lang: "fr" },
            metadata: { created: "2023-02-01", active: false },
          },
        ] as User[],
        pagination: { page: 1, total: 2 },
      },
      meta: { timestamp: "2025-01-01T00:00:00Z" },
    };

    const processedData: ProcessedUser[] = apiResponse.data.users
      .filter((user) => user.metadata.active)
      .map((user) => {
        const picked = pickDeep(user, "id", "profile.name", "settings.theme");
        return {
          ...picked,
          summary: {
            displayName: user.profile.name,
            isActive: user.metadata.active,
          },
        };
      });

    const userIndex = keyBy(processedData, "id");
    const finalConfig = mergeDeep(
      { users: userIndex },
      { meta: pick(apiResponse.meta, "timestamp") },
    );

    expect(processedData.length).toBe(1);
    expect(finalConfig.users["user-1"].summary.displayName).toBe("Alice");
    expect(has(finalConfig, "meta.timestamp")).toBe(true);
  });
});
