/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Assert;
import org.junit.Test;

/**
 * If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
 * If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
 * <p/>
 * NOTE: Do not count spaces or hyphens.
 *
 * @author barreiro
 */
public class Problem017Test extends ProjectEulerAbstractTest {

    // String arrays for debug purposes
    private static final String[] LOOKUP = {"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"};
    private static final String[] LOOKUP10 = {"zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"};

    // Prints the numbers in natural language for debug purpose using the same algorithm
    private static String[] naturalLanguage(int start, int end) {
        String[] results = new String[(end - start + 1)];
        for (int l = start, k = l; l <= end; k = ++l) {
            StringBuilder sb = new StringBuilder();

            if (k / 1000 > 0) {
                sb.append(LOOKUP[k / 1000]).append(" thousand and ");
                if ((k %= 1000) == 0) {
                    sb.setLength(sb.length() - "and ".length());
                    results[l - start] = sb.toString().trim();
                    continue;
                }
            }
            if (k / 100 > 0) {
                sb.append(LOOKUP[k / 100]).append(" hundred and ");
                if ((k %= 100) == 0) {
                    sb.setLength(sb.length() - "and ".length());
                    results[l - start] = sb.toString().trim();
                    continue;
                }
            }
            if (k / 10 <= 1) {
                sb.append(LOOKUP[k]);
            } else {
                if (k % 10 == 0) {
                    sb.append(LOOKUP10[k / 10]);
                } else {
                    sb.append(LOOKUP10[k / 10]).append(' ').append(LOOKUP[k % 10]);
                }
            }
            results[l - start] = sb.toString().trim();
        }
        return results;
    }

    /* --- */

    @Test
    public void test() {
        Assert.assertEquals(21124, new Solver017().solve());
    }

    @Test
    public void example() {
        Assert.assertEquals(19, new Solver017(5).solve());
    }

    /* --- */

    @Test
    public void natural1() {
        String[] results = naturalLanguage(19, 21);
        Assert.assertArrayEquals(new String[]{"nineteen", "twenty", "twenty one"}, results);
    }

    @Test
    public void natural2() {
        String[] results = naturalLanguage(99, 101);
        Assert.assertArrayEquals(new String[]{"ninety nine", "one hundred", "one hundred and one"}, results);
    }

    @Test
    public void natural3() {
        String[] results = naturalLanguage(199, 201);
        Assert.assertArrayEquals(new String[]{"one hundred and ninety nine", "two hundred", "two hundred and one"}, results);
    }

    @Test
    public void natural4() {
        String[] results = naturalLanguage(999, 1001);
        Assert.assertArrayEquals(new String[]{"nine hundred and ninety nine", "one thousand", "one thousand and one"}, results);
    }
}
