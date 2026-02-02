## 2026-02-02 - Fuzzy Search Optimization
**Learning:** In fuzzy search algorithms using Levenshtein distance, a length-based heuristic check (`abs(lenA - lenB) > maxDistance`) can provide a 3x-100x speedup by skipping expensive distance calculations for obvious non-matches.
**Action:** Always implement length difference pruning before running full edit distance algorithms when a threshold is available.
