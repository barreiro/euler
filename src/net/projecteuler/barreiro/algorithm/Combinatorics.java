/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import static java.lang.Math.min;

/**
 * Static class with util methods for solving problems.
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

}
