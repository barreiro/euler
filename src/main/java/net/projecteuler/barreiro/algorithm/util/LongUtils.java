/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm.util;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.LongStream;

import static java.util.Arrays.copyOf;
import static java.util.Arrays.stream;
import static java.util.stream.IntStream.range;

/**
 * Utility methods to work with long.
 *
 * @author barreiro
 */
public final class LongUtils {

    private static final long[] POW10 = new long[]{
            1,
            10,
            100,
            1000,
            10000,
            100000,
            1000000,
            10000000,
            100000000,
            1000000000,
            10000000000L,
            100000000000L,
            1000000000000L,
            10000000000000L,
            100000000000000L,
            1000000000000000L,
            10000000000000000L,
            100000000000000000L,
            1000000000000000000L
    };

    private LongUtils() {
    }

    /**
     * Default value used as base for the numeric system. Used in methods that make array-based calculations.
     *
     * @return Decimal system
     */
    private static int defaultRadix() {
        return 10;
    }

    /**
     * Test if a long is odd.
     *
     * @return true if value is not multiple of two
     */
    public static boolean odd(long l) {
        return l % 2 != 0;
    }

    /**
     * Greatest common divisor using Euclides algorithm.
     *
     * @return The greatest common divisor
     */
    public static long gcd(long a, long b) {
        while ( b != 0 ) {
            long n = a % b;
            a = b;
            b = n;
        }
        return a;
    }

    /**
     * Simple method to calculate the factorial of small values. No checks are performed. Use with caution.
     *
     * @return The factorial of the argument
     */
    public static long factorial(long l) {
        if ( l <= 2 ) {
            return l;
        }
        return l * factorial( l - 1 );
    }

    /**
     * Convenience method to calculate the power.
     *
     * @return base^exp
     */
    public static long pow(long base, long exp) {
        if ( base == 0 ) {
            return exp == 0 ? 1 : 0;
        }
        if ( base == 1 ) {
            return base;
        }
        if ( base == 2 ) {
            return 1 << exp;
        }
        if ( base == 10 ) {
            return pow10( exp );
        }

        if ( exp == 0 ) {
            return 1;
        }
        if ( exp == 1 ) {
            return base;
        }
        if ( exp == 2 ) {
            return base * base;
        }

        // Perform exp by squaring, although could resort to Math.pow( base, exp );
        long result = 1;
        for ( ; exp != 0; exp /= 2, base *= base ) {
            if ( exp % 2 != 0 ) {
                result *= base;
            }
        }
        return result;
    }

    /**
     * Convenience method to calculate the power when in base 10.
     *
     * @return base^exp
     */
    public static long pow10(long exp) {
        if ( exp < POW10.length ) {
            return POW10[(int) exp];
        }
        return (long) Math.pow( 10, exp );
    }

    /**
     * Calculates an approximate of the square root
     *
     * @return sqrt(value)
     */
    public static long intSqrt(long value) {
        long result = 0, one = 1L << 30;

        // "one" starts at the highest power of four <= than the argument
        while ( one > value ) {
            one >>= 2;
        }

        for ( ; one != 0; result >>= 1, one >>= 2 ) {
            if ( value >= result + one ) {
                value = value - ( result + one );
                result = result + one * 2;
            }
        }

        // Rounding to nearest integer
        return value > result ? result + 1 : result;
    }

    // --- //

    /**
     * Tests if a given number is a palindrome, i.e. it's digits read the same both ways
     *
     * @return true if number is a palindrome
     */
    public static boolean isPalindrome(long l) {
        return isPalindrome( toDigits( l ) );
    }

    /**
     * Tests if a given number is a palindrome in a given base, i.e. it's digits read the same both ways
     *
     * @return true if number is a palindrome
     */
    public static boolean isPalindrome(long l, int radix) {
        return isPalindrome( toDigits( l, radix ) );
    }

    /**
     * Tests if a given number is a palindrome, i.e. it's digits read the same both ways
     *
     * @return true if number is a palindrome
     */
    public static boolean isPalindrome(long... digits) {
        for ( int i = 0; i * 2 < digits.length; i++ ) {
            if ( digits[i] != digits[digits.length - i - 1] ) {
                return false;
            }
        }
        return true;
    }

    // --- //

    /**
     * Decompose a long value into a sequence of digits.
     *
     * @param l Value to convert
     * @return An array with the digits that form the number, less significant first
     */
    public static LongStream toDigitsStream(long l) {
        return stream( toDigits( l ) );
    }

    /**
     * Decompose a long value into a sequence of digits.
     *
     * @return An array with the digits that form the number, less significant first
     */
    public static long[] toDigits(long l) {
        return toDigits( l, defaultRadix() );
    }

    /**
     * Decompose a long value into a sequence of digits.
     *
     * @return An array with the digits that form the number, less significant first
     */
    public static long[] toDigits(long l, int radix) {
        List<Long> digits = new ArrayList<>();
        for ( ; l >= radix; l /= radix ) {
            digits.add( l % radix );
        }
        digits.add( l );
        return digits.stream().mapToLong( x -> x ).toArray();
    }

    /**
     * Compose a long value from a sequence of digits. There is the risk of overflow. Use carefully.
     *
     * @param digits Array of digits to convert from, less significant first
     * @return A long composed of the digits in the array
     */
    public static long fromDigits(long... digits) {
        return fromDigits( digits, 0, digits.length );
    }

    /**
     * Compose a long value from a sequence of digits. There is the risk of overflow. Use carefully.
     *
     * @param digits Array of digits to convert from, less significant first
     * @param from   Starting index on the array
     * @param to     Last index on the array (not included on the result)
     * @return A long composed of the digits in the array
     */
    public static long fromDigits(long[] digits, int from, int to) {
        return fromDigits( digits, from, to, defaultRadix() );
    }

    /**
     * Compose a long value from a sequence of digits. There is the risk of overflow. Use carefully.
     *
     * @param digits Array of digits to convert from, less significant first
     * @param from   Starting index on the array
     * @param to     Last index on the array (not included on the result)
     * @param radix  Base for the numeric system
     * @return A long composed of the digits in the array
     */
    public static long fromDigits(long[] digits, int from, int to, int radix) {
        long result = 0;
        for ( int i = from; i < to; i++ ) {
            result += digits[i - from] * pow( radix, i );
        }
        return result;
    }

    /**
     * Multiply two long numbers represented as an array of digits, less significant first.
     * There is not the risk of carry overflow, even for big numbers. Result may contain trailing zeros.
     *
     * @return Result of the multiplication of a and b
     */
    public static long[] multiplication(long[] a, long[] b) {
        return multiplication( a, b, defaultRadix() );
    }

    /**
     * Multiply two long numbers represented as an array of digits, less significant first.
     * There is not the risk of carry overflow, even for big numbers. Result may contain trailing zeros.
     *
     * @return Result of the multiplication of a and b
     */
    public static long[] multiplication(long[] a, long[] b, int radix) {
        long[] result = new long[a.length + b.length];
        range( 0, a.length ).forEach( i -> range( 0, b.length ).forEach( j -> result[i + j] += a[i] * b[j] ) );

        for ( int carry = 0, i = 0; i < result.length; i++ ) {
            result[i] += carry;
            carry = (int) result[i] / radix;
            result[i] -= carry * radix;
        }
        return result;
    }

    /**
     * Add two long numbers represented as an array of digits, less significant first. All arrays must have the same size.
     * If sum does not fit the result array a new one will be created and returned..
     *
     * @return Result of the sum of a and b
     */
    public static long[] addition(long[] a, long[] b, long[] result) {
        return addition( a, b, result, defaultRadix() );
    }

    /**
     * Add two long numbers represented as an array of digits, less significant first. All arrays must have the same size.
     * If sum does not fit the result array a new one will be created and returned..
     *
     * @return Result of the sum of a and b
     */
    public static long[] addition(long[] a, long[] b, long[] result, int radix) {
        for ( int i = 0; i < result.length; i++ ) {
            result[i] = a[i] + b[i];
        }

        long carry = 0;
        for ( int i = 0; i < result.length; i++ ) {
            result[i] += carry;
            carry = (int) result[i] / radix;
            result[i] -= carry * radix;
        }
        if ( carry != 0 ) {
            // Expand the result as needed
            result = copyOf( result, result.length + 1 );
            result[result.length - 1] = carry;
        }
        return result;
    }

    // --- //

    /**
     * Modular power.
     *
     * @param base Number to use as base
     * @param exp  Number to use as exponent
     * @param mod  Number to use as modulus
     * @return The value of the operation (base^exp) mod modulus.
     */
    public static long powerMod(long base, long exp, long mod) {
        long result = 1;
        base %= mod;
        while ( exp > 0 ) {
            if ( ( exp & 1 ) != 0 ) {
                result = ( result * base ) % mod;
            }
            exp >>= 1;
            base = base * base % mod;
        }
        return result < 0 ? result + mod : result;
    }

}
