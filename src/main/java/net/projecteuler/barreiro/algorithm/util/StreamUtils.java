/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm.util;

import java.util.Iterator;
import java.util.List;
import java.util.PrimitiveIterator;
import java.util.Set;
import java.util.stream.IntStream;
import java.util.stream.LongStream;
import java.util.stream.Stream;

import static java.util.Arrays.stream;
import static java.util.Spliterator.IMMUTABLE;
import static java.util.Spliterator.NONNULL;
import static java.util.Spliterators.spliteratorUnknownSize;
import static java.util.stream.Collectors.toList;
import static java.util.stream.Collectors.toSet;
import static java.util.stream.LongStream.iterate;
import static java.util.stream.StreamSupport.longStream;
import static java.util.stream.StreamSupport.stream;

/**
 * Utility methods to work with streams.
 *
 * @author barreiro
 */
public final class StreamUtils {

    private StreamUtils() {
    }

    /**
     * Utility Method to create a stream from an iterator. Elements are added lazily.
     *
     * @param iterator Iterator to use
     * @return A long stream with the iterated longs
     */
    public static LongStream lazyStream(PrimitiveIterator.OfLong iterator) {
        return longStream( spliteratorUnknownSize( iterator, IMMUTABLE | NONNULL ), false );
    }

    /**
     * Utility Method to create a stream from an iterator. Elements are added lazily.
     *
     * @param iterator Iterator to use
     * @return A stream with the iterated objects
     */
    public static <E> Stream<E> lazyStream(Iterator<E> iterator) {
        return stream( spliteratorUnknownSize( iterator, IMMUTABLE | NONNULL ), false );
    }

    /**
     * Creates a stream is descending order
     *
     * @param startExclusive Highest element not in the stream
     * @param endInclusive   Lowest Element
     * @return A stream with the specified range
     */
    public static LongStream rangeReverse(long startExclusive, long endInclusive) {
        return iterate( startExclusive - 1, l -> l - 1 ).limit( startExclusive - endInclusive );
    }

    /**
     * Creates an infinite parallel long stream.
     *
     * @return An infinite stream
     */
    public static LongStream infiniteParallelStream() {
        return iterate( 1L, l -> l + 1 ).parallel();
    }

    // --- //

    /**
     * Returns the highest value on a stream of longs
     *
     * @param stream the Stream of Long to find the maximum of
     * @return the highest value on the stream
     */
    public static long maxLong(Stream<Long> stream) {
        return stream.mapToLong( Long::valueOf ).max().getAsLong();
    }

    /**
     * Returns the highest value on a stream of longs
     *
     * @param stream the LongStream to find the maximum of
     * @return the highest value on the stream
     */
    public static long maxLong(LongStream stream) {
        return stream.max().getAsLong();
    }

    /**
     * Returns the first value on a stream of longs
     *
     * @param stream the LongStream to find the first value of
     * @return the first value on the stream
     */
    public static long firstLong(LongStream stream) {
        return stream.findFirst().getAsLong();
    }

    // --- //

    /**
     * Returns a List view of a LongStream
     *
     * @param stream the LongStream to convert to a List
     * @return a List view
     */
    public static List<Long> longList(LongStream stream) {
        return stream.mapToObj( Long::valueOf ).collect( toList() );
    }

    /**
     * Returns a Set view of a LongStream
     *
     * @param stream the LongStream to convert to a Set
     * @return a Set view
     */
    public static Set<Long> longSet(LongStream stream) {
        return stream.mapToObj( Long::valueOf ).collect( toSet() );
    }

    /**
     * Returns a Set view of an IntStream
     *
     * @param stream the IntStream to convert to a Set
     * @return a Set view
     */
    public static Set<Integer> intSet(IntStream stream) {
        return stream.mapToObj( Integer::valueOf ).collect( toSet() );
    }

    /**
     * Returns an array view of a Stream of Long
     *
     * @param stream the Stream of Long to convert to an array
     * @return an array view
     */
    public static long[] longArray(Stream<Long> stream) {
        return stream.mapToLong( Long::valueOf ).toArray();
    }

    /**
     * Returns a IntStream from a String[]
     *
     * @param array A string array with numeric values
     * @return a IntStream from the values in the array
     */
    public static IntStream intStream(String[] array) {
        return stream( array ).mapToInt( Integer::valueOf );
    }

}
