/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
 * What is the sum of the digits of the number 2^1000?
 *
 * @author barreiro
 */
public class Problem016Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 1366, new Solver016().solve() );
    }

    @Test
    public void example() {
        assertEquals( 26, new Solver016( 15 ).solve() );
    }

    @Test
    public void minimal0() {
        assertEquals( 1, new Solver016( 0 ).solve() );
    }

    @Test
    public void minimal1() {
        assertEquals( 2, new Solver016( 1 ).solve() );
    }

    @Test
    public void minimal2() {
        assertEquals( 7, new Solver016( 4 ).solve() );
    }

    @Test
    public void minimal3() {
        assertEquals( 13, new Solver016( 8 ).solve() );
    }

    @Test
    public void minimal4() {
        assertEquals( 7, new Solver016( 10 ).solve() );
    }

    @Test
    public void minimal5() {
        assertEquals( 62, new Solver016( 33 ).solve() );
    }

    @Test
    public void minimal6() {
        assertEquals( 86, new Solver016( 65 ).solve() );
    }

    @Test
    public void minimal7() {
        assertEquals( 115, new Solver016( 100 ).solve() );
    }

    @Test
    public void minimal8() {
        assertEquals( 256, new Solver016( 200 ).solve() );
    }

    @Test
    public void minimal9() {
        assertEquals( 679, new Solver016( 500 ).solve() );
    }

}
