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

    /**
     * Table for fast lookups of powers of 10
     */
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
    
    /**
     * Default value used as base for the numeric system. Used in methods that make array-based calculations. Default to the decimal system.
     */
    private static final int DEFAULT_RADIX = 10;

    private LongUtils() {
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
        long one = a;
        long two = b;
        while ( two != 0 ) {
            long remainder = one % two;
            one = two;
            two = remainder;
        }
        return one;
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

        // Perform exp by squaring, although could resort to Math.pow( base, exp )
        return squaring( base, exp );
    }

    private static long squaring(long base, long exp) {
        long result = 1;
        long squaringBase = base;
        long squaringExp = exp;
        for ( ; squaringExp != 0; squaringExp /= 2, squaringBase *= squaringBase ) {
            if ( squaringExp % 2 != 0 ) {
                result *= squaringBase;
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
        long result = 0;
        long one = 1L << 30;

        // "one" starts at the highest power of four <= than the argument
        while ( one > value ) {
            one >>= 2;
        }

        long approximation = value;
        for ( ; one != 0; result >>= 1, one >>= 2 ) {
            if ( approximation >= result + one ) {
                approximation -= result + one;
                result = result + ( one << 1 );
            }
        }

        // Rounding to nearest integer
        return approximation > result ? result + 1 : result;
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
        return toDigits( l, DEFAULT_RADIX );
    }

    /**
     * Decompose a long value into a sequence of digits.
     *
     * @return An array with the digits that form the number, less significant first
     */
    public static long[] toDigits(long l, int radix) {
        List<Long> digits = new ArrayList<>();
        long value = l;
        for ( ; value >= radix; value /= radix ) {
            digits.add( value % radix );
        }
        digits.add( value );
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
        return fromDigits( digits, from, to, DEFAULT_RADIX );
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
        return multiplication( a, b, DEFAULT_RADIX );
    }

    /**
     * Multiply two long numbers represented as an array of digits, less significant first.
     * There is not the risk of carry overflow, even for big numbers. Result may contain trailing zeros.
     *
     * @return Result of the multiplication of a and b
     */
    public static long[] multiplication(long[] a, long[] b, int radix) {
        long[] result = new long[a.length + b.length];
        range( 0, a.length ).forEach( i -> range( 0, b.length ).forEach( j -> result[i + j] = result[i + j] + a[i] * b[j] ) );

        long carry = 0;
        for ( int i = 0; i < result.length; i++ ) {
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
        return addition( a, b, result, DEFAULT_RADIX );
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
            carry = result[i] / radix;
            result[i] -= carry * radix;
        }
        if ( carry == 0 ) {
            return result;
        }
        // Expand the result as needed
        long[] expanded = copyOf( result, result.length + 1 );
        expanded[result.length] = carry;
        return expanded;
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
        long b = base % mod;
        long e = exp;
        while ( e > 0 ) {
            if ( ( e & 1 ) != 0 ) {
                result = ( result * b ) % mod;
            }
            e >>= 1;
            b = b * b % mod;
        }
        return result < 0 ? result + mod : result;
    }
}
