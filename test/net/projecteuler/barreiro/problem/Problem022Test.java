/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.
 * <p>
 * For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.
 * <p>
 * What is the total of all the name scores in the file?
 *
 * @author barreiro
 */
public class Problem022Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals(871198282, new Solver022().solve());
    }

    @Test
    public void example1() {
        assertEquals(53, new Solver022(1, "COLIN").solve());
    }

    @Test
    public void example2() {
        assertEquals(26819198, new Solver022(938).solve());
    }

    @Test
    public void limitedData() {
        assertEquals(496, new Solver022(5).solve());
    }

    @Test
    public void self1() {
        assertEquals(61, new Solver022(1, "LUIS").solve());
    }

    @Test
    public void self2() {
        assertEquals(86, new Solver022(1, "BARREIRO").solve());
    }

    @Test
    public void self3() {
        assertEquals(208, new Solver022(2, "\"LUIS\",\"BARREIRO\"").solve());
    }

}
