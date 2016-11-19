/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
 * If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
 * <p>
 * For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
 * <p>
 * Evaluate the sum of all the amicable numbers under 10000.
 *
 * @author barreiro
 */
public class Problem021Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 31626, new Solver021().solve() );
    }

    @Test
    public void example() {
        assertEquals( 504, new Solver021( 300 ).solve() );
    }

    @Test
    public void big() {
        assertEquals( 2896242, new Solver021( 200000 ).solve() );
    }

}
