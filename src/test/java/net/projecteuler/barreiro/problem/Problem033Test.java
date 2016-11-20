/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static net.projecteuler.barreiro.problem.Solver033.naiveCancellation;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

/**
 * The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.
 * <p>
 * We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
 * <p>
 * There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.
 * <p>
 * If the product of these four fractions is given in its lowest common terms, find the value of the denominator.
 *
 * @author barreiro
 */
public class Problem033Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 100, new Solver033().solve() );
    }

    @Test
    public void minimal() {
        assertEquals( 1, new Solver033( 1 ).solve() );
    }

    @Test
    public void medium1() {
        assertEquals( 4, new Solver033( 65 ).solve() );
    }

    @Test
    public void medium2() {
        assertEquals( 10, new Solver033( 66 ).solve() );
    }

    @Test
    public void big() {
        assertEquals( 100, new Solver033( 1000 ).solve() );
    }

    // --- //

    @Test
    public void testNaiveCancellation() {
        assertTrue( naiveCancellation( 16, 64, 6 ) );
        assertTrue( naiveCancellation( 26, 65, 6 ) );
        assertTrue( naiveCancellation( 19, 95, 9 ) );
        assertTrue( naiveCancellation( 49, 98, 9 ) );

        assertFalse( naiveCancellation( 9, 95, 9 ) );
        assertFalse( naiveCancellation( 39, 95, 9 ) );
        assertFalse( naiveCancellation( 39, 98, 9 ) );
        assertFalse( naiveCancellation( 59, 98, 9 ) );
    }

}
