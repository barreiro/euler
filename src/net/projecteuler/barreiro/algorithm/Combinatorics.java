/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import java.util.Set;

import static java.lang.Math.min;

/**
 * Static class with util methods for solving problems related to combinatorial logic .
 *
 * @author barreiro
 */
public final class Combinatorics {

    private Combinatorics() {
    }

    /**
     * Method for calculation the combinations of a certain number of elements in a total number of places.
     * Uses recursion instead of the formula with factorials.
     *
     * @param totalL  Number of places
     * @param chooseL Number of elements
     * @return The number of combinations
     */
    public static long choose(long totalL, long chooseL) {
        int total = (int) totalL, choose = (int) min(chooseL, totalL - chooseL); // Take full advantage of symmetry
        return choose < 0 ? 0 : choose == 0 ? 1 : choose(total, choose, new long[total + 1][choose + 1]);
    }

    private static long choose(int total, int choose, long[][] cache) {
        if ((choose = min(choose, total - choose)) == 1) return total;
        if (cache[total][choose] != 0) return cache[total][choose];

        long value = choose(total - 1, choose - 1, cache) + choose(total - 1, choose, cache);

        cache[total][choose] = value;
        return value;
    }

    /**
     * Calculates the number of integer partition of a value, given a set of constrains.
     *
     * @param valueL     Number to partition
     * @param constrains Set of values that are allowed on the partition
     * @return The number of different partitions
     */
    public static long partition(long valueL, Set<Integer> constrains) {
        int value = (int) valueL;
        return partition(value, value, 0, constrains, new long[value + 1][value + 1]);
    }

    private static long partition(int remaining, int total, long sum, Set<Integer> constrains, long[][] cache) {
        if (remaining == 0) return 1;
        if (cache[remaining][total] != 0) return cache[remaining][total];

        long l = 0;
        for (int c = min(remaining, total); c > 0; c--) {
            if (c <= remaining && constrains.contains(c)) l += partition(remaining - c, c, sum + c, constrains, cache);
        }
        cache[remaining][total] = l;
        return l;
    }


}
