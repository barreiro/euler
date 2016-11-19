/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static net.projecteuler.barreiro.problem.Solver004.isPalindromic;
import static org.junit.Assert.assertEquals;

/**
 * A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
 * Find the largest palindrome made from the product of two 3-digit numbers.
 *
 * @author barreiro
 */
public class Problem004Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 906609, new Solver004().solve() );
    }

    @Test
    public void example() {
        assertEquals( 9009, new Solver004( 2 ).solve() );
    }

    @Test
    public void minimal() {
        assertEquals( 9, new Solver004( 1 ).solve() );
    }

    @Test
    public void big() {
        assertEquals( 99000099, new Solver004( 4 ).solve() );
    }

    // --- //

    @Test
    public void palindromic0() {
        assertEquals( false, isPalindromic( 15 ) );
    }

    @Test
    public void palindromic1() {
        assertEquals( true, isPalindromic( 88 ) );
    }

    @Test
    public void palindromic2() {
        assertEquals( false, isPalindromic( 15846 ) );
    }

    @Test
    public void palindromic3() {
        assertEquals( true, isPalindromic( 84048 ) );
    }
}
