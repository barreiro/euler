/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
 * Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
 *
 * @author barreiro
 */
public class Problem008Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 23514624000L, new Solver008().solve() );
    }

    @Test
    public void example() {
        assertEquals( 5832, new Solver008( 4 ).solve() );
    }

    @Test
    public void minimal() {
        assertEquals( 8, new Solver008( 1, "28" ).solve() );
    }

    @Test
    public void big() {
        assertEquals( 240789749760000L, new Solver008( 20 ).solve() );
    }

}
