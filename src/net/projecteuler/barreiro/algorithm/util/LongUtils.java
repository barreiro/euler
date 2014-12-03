package net.projecteuler.barreiro.algorithm.util;

import java.util.LinkedList;
import java.util.function.LongBinaryOperator;
import java.util.function.ToLongFunction;

import static java.util.stream.IntStream.range;

/**
 * Util functions to work with long.
 *
 * @author barreiro
 */
public final class LongUtils {

    private static final int RADIX = 10;

    private LongUtils() {
    }

    /**
     * Test if a long is not zero.
     *
     * @param l Value to test
     * @return true if value not equal to zero
     */
    public static boolean isntZero(long l) {
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
     * Decompose a long value into a sequence of digits.
     *
     * @param value Value to convert
     * @return An array with the digits that form the number, less significant first
     */
    public static long[] toDigits(long value) {
        LinkedList<Long> digits = new LinkedList<>();
        for (; value >= RADIX; value /= RADIX) {
            digits.addFirst(value % RADIX);
        }
        digits.addFirst(value);
        return digits.stream().mapToLong(l -> l).toArray();
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
        long[] result = new long[a.length + b.length];
        range(0, a.length).forEach(i -> range(0, b.length).forEach(j -> result[i + j] += a[i] * b[j]));

        for (int carry = 0, i = 0; i < result.length; i++) {
            result[i] += carry;
            carry = (int) result[i] / RADIX;
            result[i] -= carry * RADIX;
        }
        return result;
    }

    /* --- */

    /**
     * Provides a product function to be used in reduce operations.
     *
     * @return A product operator
     */
    public static LongBinaryOperator product() {
        return (l1, l2) -> l1 * l2;
    }

    /**
     * Provides a function to calculate the product of digits in a char sequence
     *
     * @return A function
     */
    public static ToLongFunction<CharSequence> charProduct() {
        return s -> s.chars().mapToLong(c -> c - '0').reduce(1, product());
    }

    /**
     * Provides a function to calculate the product of letters in a char sequence of uppercase chars.
     *
     * @return A function
     */
    public static ToLongFunction<CharSequence> letterSum() {
        return s -> s.chars().mapToLong(c -> c - 'A' + 1).reduce(0, Long::sum);
    }

}
