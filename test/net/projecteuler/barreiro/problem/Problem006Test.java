/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Assert;
import org.junit.Test;

/**
 * The sum of the squares of the first ten natural numbers is, 1^2 + 2^2 + ... + 10^2 = 385
 * The square of the sum of the first ten natural numbers is, (1 + 2 + ... + 10)^2 = 55^2 = 3025
 * Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
 * <p/>
 * Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
 *
 * @author barreiro
 */
public class Problem006Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        Assert.assertEquals(25164150, new Solver006().solve());
    }

    @Test
    public void example() {
        Assert.assertEquals(2640, new Solver006(10).solve());
    }

    @Test
    public void minimal() {
        Assert.assertEquals(170, new Solver006(5).solve());
    }

    @Test
    public void big() {
        Assert.assertEquals(401323300, new Solver006(200).solve());
    }

}
