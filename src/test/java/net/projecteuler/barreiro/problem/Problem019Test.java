/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

/**
 * You are given the following information, but you may prefer to do some research for yourself.
 * <p>
 * 1 Jan 1900 was a Monday.
 * Thirty days has September, April, June and November. All the rest have thirty-one, Saving February alone, Which has twenty-eight, rain or shine. And on leap years, twenty-nine.
 * A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
 * <p>
 * How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
 *
 * @author barreiro
 */
public class Problem019Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 171, new Solver019().solve() );
    }

    @Test
    public void minimal1() {
        assertEquals( 2, new Solver019( 1 ).solve() );
    }

    @Test
    public void minimal2() {
        assertEquals( 3, new Solver019( 2 ).solve() );
    }

    @Test
    public void minimal3() {
        assertEquals( 6, new Solver019( 3 ).solve() );
    }

    @Test
    public void minimal4() {
        assertEquals( 7, new Solver019( 4 ).solve() );
    }

    @Test
    public void reduced() {
        assertEquals( 17, new Solver019( 10 ).solve() );
    }

    @Test
    public void exaggerated() {
        assertEquals( 17200, new Solver019( 10000 ).solve() );
    }

    // --- //

    @Test
    public void leap1() {
        assertFalse( Solver019.isLeap( 2001 ) );
    }

    @Test
    public void leap2() {
        assertTrue( Solver019.isLeap( 2012 ) );
    }

    @Test
    public void leap3() {
        assertTrue( Solver019.isLeap( 2000 ) );
    }

    @Test
    public void leap4() {
        assertFalse( Solver019.isLeap( 1900 ) );
    }

}
