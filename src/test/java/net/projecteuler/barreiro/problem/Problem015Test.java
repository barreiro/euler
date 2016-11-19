/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * Starting in the top left corner of a 2x2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
 * How many such routes are there through a 20x20 grid?
 *
 * @author barreiro
 */
public class Problem015Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 137846528820L, new Solver015().solve() );
    }

    @Test
    public void example() {
        assertEquals( 6, new Solver015( 2 ).solve() );
    }

    @Test
    public void big() {
        assertEquals( 118264581564861424L, new Solver015( 30 ).solve() );
    }

    @Test
    public void minimal1() {
        assertEquals( 2, new Solver015( 1 ).solve() );
    }

    @Test
    public void minimal2() {
        assertEquals( 20, new Solver015( 3 ).solve() );
    }

    @Test
    public void minimal3() {
        assertEquals( 252, new Solver015( 5 ).solve() );
    }

    @Test
    public void minimal4() {
        assertEquals( 184756, new Solver015( 10 ).solve() );
    }

}
