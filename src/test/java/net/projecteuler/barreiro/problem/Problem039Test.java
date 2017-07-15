/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.
 * <p>
 * {20,48,52}, {24,45,51}, {30,40,50}
 * <p>
 * For which value of p ≤ 1000, is the number of solutions maximised?
 *
 * @author barreiro
 */
public class Problem039Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 840, new Solver039().solve() );
    }

    @Test
    public void minimal() {
        assertEquals( 12, new Solver039( 14 ).solve() );
    }

    @Test
    public void small() {
        assertEquals( 60, new Solver039( 66 ).solve() );
    }

    @Test
    public void example() {
        assertEquals( 120, new Solver039( 150 ).solve() );
    }

    @Test
    public void big() {
        assertEquals( 7560, new Solver039( 10000 ).solve() );
    }
}
