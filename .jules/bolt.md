## 2026-02-02 - Fuzzy Search Optimization
**Learning:** In fuzzy search algorithms using Levenshtein distance, a length-based heuristic check (`abs(lenA - lenB) > maxDistance`) can provide a 3x-100x speedup by skipping expensive distance calculations for obvious non-matches.
**Action:** Always implement length difference pruning before running full edit distance algorithms when a threshold is available.

## 2026-03-16 - [Iterators and Manual While loops]
**Learning:** [When optimizing functions that consume iterables/generators, using a manual while loop and `iterator.next()` instead of a `for...of` loop is a critical regression. The `for...of` loop natively handles early termination by automatically calling `.return()` on the underlying iterable if aborted early. A manual while loop bypasses this, leading to resource/memory leaks in generators holding cleanup logic.]
**Action:** [Avoid replacing `for...of` loops with manual while loops when consuming iterables unless explicitly implementing complex generator handling with try/finally and `.return()` propagation. Trust `for...of` for safe iterable consumption.]
