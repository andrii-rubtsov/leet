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

❯ bat src/main/java/_2403_count_unreachable_pairs_of_nodes_in_an_undirected_graph/Solution.java
   1   /**
   2    * 2403. Count Unreachable Pairs of Nodes in an Undirected Graph
   3    * https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/
   4    */
   5   package _2403_count_unreachable_pairs_of_nodes_in_an_undirected_graph;
   6   
   7   
   8   import java.util.*;
   9   
  10   class Solution {
  11       public long countPairs(int n, int[][] edges) {
  12           
  13       }
  14   }

❯ bat src/test/java/_2403_count_unreachable_pairs_of_nodes_in_an_undirected_graph/SolutionTest.java
   1   package _2403_count_unreachable_pairs_of_nodes_in_an_undirected_graph;
   2   
   3   import org.junit.jupiter.api.Test;
   4   
   5   import static common.utils.TestUtils.*;
   6   import static org.junit.jupiter.api.Assertions.*;
   7   
   8   
   9   class SolutionTest {
  10   
  11       @Test
  12       public void test_0() {
  13           //int actual = new Solution().xxx(
  14           //        6,
  15           //        array2d("[[0,1],[1,3],[2,3],[4,0],[4,5]]"));
  16           //assertEquals(3, actual);
  17       }
  18   
  19   }


```