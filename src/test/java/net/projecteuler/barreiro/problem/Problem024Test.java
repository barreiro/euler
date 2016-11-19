/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4.
 * If all of the permutations are listed numerically or alphabetically, we call it lexicographic order.
 * The lexicographic permutations of 0, 1 and 2 are:
 * <p>
 * 012   021   102   120   201   210
 * <p>
 * What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
 *
 * @author barreiro
 */
public class Problem024Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 2783915460L, new Solver024().solve() );
    }

    @Test
    public void example() {
        assertEquals( 201, new Solver024( 5, 3 ).solve() );
    }

    @Test
    public void minimal1() {
        assertEquals( 123456789, new Solver024( 1 ).solve() );
    }

    @Test
    public void minimal2() {
        assertEquals( 123456798, new Solver024( 2 ).solve() );
    }

    @Test
    public void veryBig() {
        assertEquals( 9876543201L, new Solver024( 3628799 ).solve() );
    }

    @Test
    public void biggest() {
        assertEquals( 9876543210L, new Solver024( 3628800 ).solve() );
    }

}
