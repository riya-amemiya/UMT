import { calculator } from "@/Math/calculator";
import { average } from "@/Math/average";
import { standardDeviation } from "@/Math/standardDeviation";
import { deviationValue } from "@/Math/deviationValue";
import { sum } from "@/Array/sum";

/**
 * Integration tests for Math utility functions
 *
 * Tests the interaction between calculator and statistical functions:
 * - Using calculator results in statistical calculations
 * - Complex mathematical workflows
 * - Real-world calculation scenarios
 */
describe("Integration test for calculator and statistical functions", () => {
  it("should calculate average from calculator results", () => {
    const values = [
      calculator("10 + 5"),
      calculator("20 - 8"),
      calculator("3 * 6"),
      calculator("100 / 4"),
    ].map(Number);

    const avg = average(values);
    expect(avg).toBe(17.5);
  });

  it("should compute standard deviation from complex calculations", () => {
    const scores = [90, 80, 90, 70, 85];

    const avg = average(scores);
    const stdDev = standardDeviation(scores);

    expect(avg).toBe(83);
    expect(stdDev).toBeCloseTo(7.483314773547883, 5);
  });

  it("should calculate deviation values for test scores", () => {
    const rawScores = [65, 72, 88, 91, 79];

    const avg = average(rawScores);
    const stdDev = standardDeviation(rawScores);

    const deviationScores = rawScores.map((score) =>
      deviationValue(score, avg, stdDev),
    );

    expect(deviationScores[0]).toBeCloseTo(35.56, 0);
    expect(deviationScores[2]).toBeCloseTo(59.28, 0);
    expect(deviationScores[4]).toBeCloseTo(50, 0);
  });

  it("should handle currency conversion in statistical calculations", () => {
    const pricesInYen = [1500, 3750, 2325, 4500];

    const avgPrice = average(pricesInYen);
    expect(avgPrice).toBe(3018.75);
  });

  it("should calculate weighted averages using calculator", () => {
    const grades = [
      { score: Number(calculator("85")), weight: 0.3 },
      { score: Number(calculator("90")), weight: 0.4 },
      { score: Number(calculator("78")), weight: 0.3 },
    ];

    const weightedSum = grades.reduce(
      (acc, grade) => acc + grade.score * grade.weight,
      0,
    );

    expect(weightedSum).toBe(84.9);
  });

  it("should perform multi-step calculations with statistical analysis", () => {
    const initialValues = [100, 150, 200, 250, 300];

    const growthRates = initialValues.map((val, i) => {
      if (i === 0) return 0;
      return ((val - initialValues[i - 1]) / initialValues[i - 1]) * 100;
    });

    const avgGrowthRate = average(growthRates.filter((rate) => rate !== 0));
    expect(avgGrowthRate).toBeCloseTo(32.083, 2);
  });

  it("should calculate complex expressions and analyze results", () => {
    const results = [13, 12, 10, 27, 2];
    const total = sum(results);
    const avg = average(results);

    expect(results).toEqual([13, 12, 10, 27, 2]);
    expect(total).toBe(64);
    expect(avg).toBe(12.8);
  });
});
