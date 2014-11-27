/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static java.util.stream.LongStream.range;

/**
 * The following iterative sequence is defined for the set of positive integers: n → n/2 (n is even) n → 3n + 1 (n is odd)
 * <p/>
 * Using the rule above and starting with 13, we generate the following sequence: 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
 * It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
 * Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
 * <p/>
 * Which starting number, under one million, produces the longest chain?
 * <p/>
 * NOTE: Once the chain starts the terms are allowed to go above one million.
 *
 * @author barreiro
 */
public class Solver014 extends ProjectEulerSolver {

    public Solver014() {
        this(1000000);
    }

    public Solver014(long n) {
        super(n);
    }

    /* --- */

    public long solve() {
        final long[] cache = new long[(int) N];
        cache[1] = 1;
        return range(2, N).reduce(1, (l1, l2) -> collatzLength(l1, cache) > collatzLength(l2, cache) ? l1 : l2);
    }

    /* --- */

    private long collatzLength(long number, long[] cache) {
        // Can't rely on the cache for everything but in many cases we can cut lots of recursion.
        int cIndex = (int) number;
        if ((number < cache.length) && (cache[cIndex] != 0)) return cache[cIndex] + 1;

        long chainLength = (number % 2 == 0 ? collatzLength(number / 2, cache) : collatzLength(3 * number + 1, cache)) + 1;

        if (number < cache.length) cache[cIndex] = chainLength;
        return chainLength;
    }

}
