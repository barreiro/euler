/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * The Fibonacci sequence is defined by the recurrence relation:
 * <p>
 * Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
 * <p>
 * Hence the first 12 terms will be:
 * F1 = 1
 * F2 = 1
 * F3 = 2
 * F4 = 3
 * F5 = 5
 * F6 = 8
 * F7 = 13
 * F8 = 21
 * F9 = 34
 * F10 = 55
 * F11 = 89
 * F12 = 144
 * The 12th term, F12, is the first term to contain three digits.
 * <p>
 * What is the first term in the Fibonacci sequence to contain 1000 digits?
 *
 * @author barreiro
 */
public class Problem025Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 4782, new Solver025().solve() );
    }

    @Test
    public void example() {
        assertEquals( 12, new Solver025( 3 ).solve() );
    }

    @Test
    public void minimal() {
        assertEquals( 7, new Solver025( 2 ).solve() );
    }

    @Test
    public void small() {
        assertEquals( 36, new Solver025( 8 ).solve() );
    }

    @Test
    public void big() {
        assertEquals( 23922, new Solver025( 5000 ).solve() );
    }

}
