/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
 * <p>
 * Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
 * <p>
 * (Please note that the palindromic number, in either base, may not include leading zeros.)
 *
 * @author barreiro
 */
public class Problem036Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 872187, new Solver036().solve() );
    }

    @Test
    public void minimal() {
        assertEquals( 1, new Solver036( 2 ).solve() );
    }

    @Test
    public void small() {
        assertEquals( 25, new Solver036( 20 ).solve() );
    }

    @Test
    public void example() {
        assertEquals( 1055, new Solver036( 586 ).solve() );
    }

    @Test
    public void big() {
        assertEquals( 25846868, new Solver036( 10000000 ).solve() );
    }

}
