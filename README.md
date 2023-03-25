# leet
Tiny CLI utility to quickly create a Java template for solving [Leetcode](https://leetcode.com)  problems

It will:
1) retrieve some metadata (problem id, title, code snippet (java)) from leetcode
2) Create a solution file from tempalte: "[location]/src/main/java/<pacakge_name>/Solution.java"
3) Create a unit test file for solution from tempalte: "[location]/src/main/java/<pacakge_name>/Solution.java"

## Example:
```bash
❯ leet https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/
 👉 Created: src/main/java/_2403_count_unreachable_pairs_of_nodes_in_an_undirected_graph/Solution.java
 👉 Created: src/test/java/_2403_count_unreachable_pairs_of_nodes_in_an_undirected_graph/SolutionTest.java

```