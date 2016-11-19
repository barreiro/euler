/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm.util;

import org.junit.Test;

import static net.projecteuler.barreiro.algorithm.util.LongUtils.addition;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.fromDigits;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.gcd;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.multiplication;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.powerMod;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.toDigits;
import static org.junit.Assert.assertEquals;

/**
 * @author barreiro
 */
public class LongUtilsTest {

    @Test
    public void gcdTest() {
        assertEquals( 6, gcd( 54, 24 ) );
        assertEquals( 6, gcd( 24, 54 ) );
    }

    @Test
    public void bigDigitConversion() {
        assertEquals( 1234567, fromDigits( toDigits( 1234567 ) ) );
    }

    @Test
    public void bigDigitMultiplicationTest() {
        assertEquals( 27, fromDigits( multiplication( toDigits( 3 ), toDigits( 9 ) ) ) );
        assertEquals( 8712, fromDigits( multiplication( toDigits( 99 ), toDigits( 88 ) ) ) );
    }

    @Test
    public void bigDigitAdditionTest() {
        assertEquals( 9, fromDigits( addition( toDigits( 4 ), toDigits( 5 ), new long[1] ) ) );
        assertEquals( 99, fromDigits( addition( toDigits( 46 ), toDigits( 53 ), new long[2] ) ) );
        assertEquals( 100, fromDigits( addition( toDigits( 46 ), toDigits( 54 ), new long[2] ) ) );
    }

    @Test
    public void modularExponentiation() {
        assertEquals( 445, powerMod( 4, 13, 497 ) );
    }

    @Test
    public void modularExponentiationSmall() {
        assertEquals( 1, powerMod( 3, 0, 7 ) );
        assertEquals( 3, powerMod( 3, 1, 7 ) );
        assertEquals( 2, powerMod( 3, 2, 7 ) );
        assertEquals( 6, powerMod( 3, 3, 7 ) );
        assertEquals( 4, powerMod( 3, 4, 7 ) );
        assertEquals( 5, powerMod( 3, 5, 7 ) );
        assertEquals( 1, powerMod( 3, 6, 7 ) );
    }

}
