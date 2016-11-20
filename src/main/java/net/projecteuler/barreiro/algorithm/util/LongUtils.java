/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm.util;

import java.util.ArrayList;
import java.util.List;
import java.util.function.LongBinaryOperator;
import java.util.function.ToLongFunction;
import java.util.stream.LongStream;

import static java.util.Arrays.copyOf;
import static java.util.stream.IntStream.range;

/**
 * Utility methods to work with long.
 *
 * @author barreiro
 */
public final class LongUtils {

    private LongUtils() {}

    /**
     * Default value used as base for the numeric system. Used in methods that make array-based calculations.
     * @return Decimal system
     */
    private static int defaultRadix() {
        return 10;
    }

    /**
     * Test if a long is not zero.
     *
     * @param l Value to test
     * @return true if value not equal to zero
     */
    public static boolean notZero(long l) {
        return l != 0;
    }

    /**
     * Test if a long is even.
     *
     * @param l Value to test
     * @return true if value is multiple of tow
     */
    public static boolean isEven(long l) {
        return l % 2 == 0;
    }

    /**
     * Test if a long is odd.
     *
     * @param l Value to test
     * @return true if value is not multiple of tow
     */
    public static boolean isOdd(long l) {
        return !isEven( l );
    }

    /**
     * Greatest common divisor using Euclides algorithm.
     *
     * @param a One of the values
     * @param b The other value
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
     * @param l Value to calculate the factorial
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
     * @param base Value to be used as base
     * @param exp  Value to be used as exponent
     * @return base^exp
     */
    public static long pow(long base, long exp) {
        return (long) Math.pow( base, exp );
    }

    // --- //

    /**
     * Decompose a long value into a sequence of digits.
     *
     * @param value Value to convert
     * @return An array with the digits that form the number, less significant first
     */
    public static long[] toDigits(long value) {
        return toDigits( value, defaultRadix() );
    }

    /**
     * Decompose a long value into a sequence of digits.
     *
     * @param value Value to convert
     * @param radix Base to use
     * @return An array with the digits that form the number, less significant first
     */
    public static long[] toDigits(long value, int radix) {
        List<Long> digits = new ArrayList<>();
        for ( ; value >= radix; value /= radix ) {
            digits.add( value % radix );
        }
        digits.add( value );
        return digits.stream().mapToLong( l -> l ).toArray();
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
        return range( from, to ).mapToLong( i -> digits[i - from] * pow( radix, i ) ).sum();
    }

    /**
     * Multiply two long numbers represented as an array of digits, less significant first.
     * There is not the risk of carry overflow, even for big numbers. Result may contain trailing zeros.
     *
     * @param a Multiplicand one
     * @param b Multiplicand two
     * @return Result of the multiplication of a and b
     */
    public static long[] multiplication(long[] a, long[] b) {
        return multiplication( a, b, defaultRadix() );
    }

    /**
     * Multiply two long numbers represented as an array of digits, less significant first.
     * There is not the risk of carry overflow, even for big numbers. Result may contain trailing zeros.
     *
     * @param a Multiplicand one
     * @param b Multiplicand two
     * @param radix Base for the numeric system
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
     * @param a      Addend one
     * @param b      Addend two
     * @param result The array where the result will be put
     * @return Result of the sum of a and b
     */
    public static long[] addition(long[] a, long[] b, long[] result) {
        return addition( a, b, result, defaultRadix() );
    }

    /**
     * Add two long numbers represented as an array of digits, less significant first. All arrays must have the same size.
     * If sum does not fit the result array a new one will be created and returned..
     *
     * @param a      Addend one
     * @param b      Addend two
     * @param result The array where the result will be put
     * @param radix  Base for the numeric system
     * @return Result of the sum of a and b
     */
    public static long[] addition(long[] a, long[] b, long[] result, int radix) {
        range( 0, result.length ).forEach( i -> result[i] = a[i] + b[i] );

        long carry = 0;
        for ( int i = 0; i < result.length; i++ ) {
            result[i] += carry;
            carry = (int) result[i] / radix;
            result[i] -= carry * radix;
        }
        if ( carry == 0 ) {
            return result;
        }

        // Expand the result as needed
        long[] r = copyOf( result, result.length + 1 );
        r[result.length] = carry;
        return r;
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

    // --- //

    /**
     * Provides a product function to be used in reduce operations.
     *
     * @return A product operator
     */
    public static LongBinaryOperator product() {
        return (l1, l2) -> l1 * l2;
    }

    /**
     * Reduces a long stream by applying the product reduction.
     *
     * @return A product operator
     */
    public static long product(LongStream stream) {
        return stream.reduce( 1, product() );
    }

    /**
     * Provides a function to calculate the product of digits in a char sequence
     *
     * @return A function
     */
    public static ToLongFunction<CharSequence> charProduct() {
        return s -> s.chars().mapToLong( c -> c - '0' ).reduce( 1, product() );
    }

    /**
     * Provides a function to calculate the product of letters in a char sequence of uppercase chars.
     *
     * @return A function
     */
    public static ToLongFunction<CharSequence> letterSum() {
        return s -> s.chars().mapToLong( c -> c - 'A' + 1 ).reduce( 0, Long::sum );
    }

}
