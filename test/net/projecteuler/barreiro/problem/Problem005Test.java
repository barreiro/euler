/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

/**
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
 *
 * @author barreiro
 */
public class Problem005Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        long solution = new Solver005().solve();
        assertTrue(check(solution, 20));
        assertEquals(232792560, solution);
    }

    @Test
    public void example() {
        assertTrue(check(new Solver005(10).solve(), 10));
    }

    @Test
    public void minimal() {
        assertTrue(check(new Solver005(6).solve(), 6));
    }

    @Test
    public void big() {
        assertTrue(check(new Solver005(40).solve(), 40));
    }

    /* --- */

    private boolean check(long solution, long number) {
        for (int i = 1; i < number; i++) {
            if (solution % i != 0) return false;
        }
        return true;
    }
}
