/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

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
        int total = (int) totalL, choose = (int) chooseL;
        return choose(total, choose, new long[total + 1][choose + 1]);
    }

    private static long choose(int total, int choose, long[][] cache) {
        if (total < choose) {
            return 0;
        }
        if (choose == 0 || choose == total) {
            return 1;
        }
        choose = Math.min(choose, total - choose); // Take full advantage of symmetry

        if (cache[total][choose] != 0) {
            return cache[total][choose];
        }
        long value = choose(total - 1, choose - 1, cache) + choose(total - 1, choose, cache);
        cache[total][choose] = value;
        return value;
    }

}
