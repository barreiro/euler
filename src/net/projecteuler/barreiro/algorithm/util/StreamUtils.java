package net.projecteuler.barreiro.algorithm.util;

import java.util.Iterator;
import java.util.PrimitiveIterator;
import java.util.stream.LongStream;
import java.util.stream.Stream;

import static java.util.Spliterator.IMMUTABLE;
import static java.util.Spliterator.NONNULL;
import static java.util.Spliterators.spliteratorUnknownSize;
import static java.util.stream.LongStream.iterate;
import static java.util.stream.StreamSupport.longStream;
import static java.util.stream.StreamSupport.stream;

/**
 * Util functions to work with streams.
 * <p>
 * Created by barreiro on 11/20/14.
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
        return longStream(spliteratorUnknownSize(iterator, IMMUTABLE | NONNULL), false);
    }

    /**
     * Utility Method to create a stream from an iterator. Elements are added lazily.
     *
     * @param iterator Iterator to use
     * @return A stream with the iterated objects
     */
    public static <E> Stream<E> lazyStream(Iterator<E> iterator) {
        return stream(spliteratorUnknownSize(iterator, IMMUTABLE | NONNULL), false);
    }

    /**
     * Creates a stream is descending order
     *
     * @param startExclusive Highest element not in the stream
     * @param endInclusive   Lowest Element
     * @return A stream with the specified range
     */
    public static LongStream rangeReverse(long startExclusive, long endInclusive) {
        return iterate(startExclusive - 1, l -> l - 1).limit(startExclusive - endInclusive);
    }

}
