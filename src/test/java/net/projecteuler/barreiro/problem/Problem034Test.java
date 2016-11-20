/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
 * <p>
 * Find the sum of all numbers which are equal to the sum of the factorial of their digits.
 * <p>
 * Note: as 1! = 1 and 2! = 2 are not sums they are not included.
 *
 * @author barreiro
 */
public class Problem034Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 40730, new Solver034().solve() );
    }

    @Test
    public void minimal() {
        assertEquals( 145, new Solver034( 150 ).solve() );
    }

    @Test
    public void big() {
        assertEquals( 40730, new Solver034( 1000000 ).solve() );
    }

}
