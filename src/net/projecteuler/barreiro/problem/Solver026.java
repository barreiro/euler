/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static net.projecteuler.barreiro.algorithm.Primes.primeFactors;
import static net.projecteuler.barreiro.algorithm.Primes.primesLessThan;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.powerMod;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.firstLong;

/**
 * A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:
 * <p>
 * 1/2	= 	0.5
 * 1/3	= 	0.(3)
 * 1/4	= 	0.25
 * 1/5	= 	0.2
 * 1/6	= 	0.1(6)
 * 1/7	= 	0.(142857)
 * 1/8	= 	0.125
 * 1/9	= 	0.(1)
 * 1/10	= 	0.1
 * <p>
 * Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.
 * <p>
 * Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
 *
 * @author barreiro
 */
public class Solver026 extends ProjectEulerSolver {

    public Solver026() {
        this(1000);
    }

    public Solver026(long n) {
        super(n);
    }

    /* --- */

    public long solve() {
        // For primes: if 10 is a primitive root modulo p, the recurring cycle is equal to p − 1; if not is a factor of p − 1
        return firstLong(primesLessThan(N).filter(Solver026::isPrimitiveRootTen));
    }

    private static boolean isPrimitiveRootTen(long p) {
        return primeFactors(p - 1).keySet().stream().noneMatch(k -> powerMod(10, (p - 1) / k, p) == 1);
    }

}
