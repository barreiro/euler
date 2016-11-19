/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
 *
 * @author barreiro
 */
public class Problem013Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 5537376230L, new Solver013().solve() );
    }

    @Test
    public void big() {
        assertEquals( 553737623039087L, new Solver013( 15 ).solve() );
    }

    @Test
    public void minimal1() {
        assertEquals( 5, new Solver013( 1 ).solve() );
    }

    @Test
    public void minimal2() {
        assertEquals( 55, new Solver013( 2 ).solve() );
    }

    @Test
    public void minimal3() {
        assertEquals( 553, new Solver013( 3 ).solve() );
    }

    @Test
    public void minimal4() {
        assertEquals( 5537, new Solver013( 4 ).solve() );
    }

    @Test
    public void minimal5() {
        assertEquals( 55373, new Solver013( 5 ).solve() );
    }

}
