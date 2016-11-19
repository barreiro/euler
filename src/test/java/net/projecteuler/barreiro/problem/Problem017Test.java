/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;

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
        String[] results = new String[( end - start + 1 )];
        for ( int l = start, k = l; l <= end; k = ++l ) {
            StringBuilder sb = new StringBuilder();

            if ( k / 1000 > 0 ) {
                sb.append( LOOKUP[k / 1000] ).append( " thousand and " );
                if ( ( k %= 1000 ) == 0 ) {
                    sb.setLength( sb.length() - "and ".length() );
                    results[l - start] = sb.toString().trim();
                    continue;
                }
            }
            if ( k / 100 > 0 ) {
                sb.append( LOOKUP[k / 100] ).append( " hundred and " );
                if ( ( k %= 100 ) == 0 ) {
                    sb.setLength( sb.length() - "and ".length() );
                    results[l - start] = sb.toString().trim();
                    continue;
                }
            }
            if ( k / 10 <= 1 ) {
                sb.append( LOOKUP[k] );
            }
            else {
                if ( k % 10 == 0 ) {
                    sb.append( LOOKUP10[k / 10] );
                }
                else {
                    sb.append( LOOKUP10[k / 10] ).append( ' ' ).append( LOOKUP[k % 10] );
                }
            }
            results[l - start] = sb.toString().trim();
        }
        return results;
    }

    // --- //

    @Test
    public void test() {
        assertEquals( 21124, new Solver017().solve() );
    }

    @Test
    public void example() {
        assertEquals( 19, new Solver017( 5 ).solve() );
    }

    // --- //

    @Test
    public void natural1() {
        assertArrayEquals( new String[]{"nineteen", "twenty", "twenty one"}, naturalLanguage( 19, 21 ) );
    }

    @Test
    public void natural2() {
        assertArrayEquals( new String[]{"ninety nine", "one hundred", "one hundred and one"}, naturalLanguage( 99, 101 ) );
    }

    @Test
    public void natural3() {
        assertArrayEquals( new String[]{"one hundred and ninety nine", "two hundred", "two hundred and one"}, naturalLanguage( 199, 201 ) );
    }

    @Test
    public void natural4() {
        assertArrayEquals( new String[]{"nine hundred and ninety nine", "one thousand", "one thousand and one"}, naturalLanguage( 999, 1001 ) );
    }
}
