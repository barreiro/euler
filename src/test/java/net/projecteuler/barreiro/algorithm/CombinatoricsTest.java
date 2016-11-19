/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import org.junit.Test;

import java.util.List;
import java.util.Set;

import static java.util.Arrays.asList;
import static java.util.stream.Collectors.toSet;
import static java.util.stream.IntStream.rangeClosed;
import static net.projecteuler.barreiro.algorithm.Combinatorics.choose;
import static net.projecteuler.barreiro.algorithm.Combinatorics.partition;
import static org.junit.Assert.assertEquals;

/**
 * @author barreiro
 */
public class CombinatoricsTest {

    @Test
    public void chooseMinimal1() {
        assertEquals( 1, choose( 1, 1 ) );
    }

    @Test
    public void chooseMinimal2() {
        assertEquals( 2, choose( 2, 1 ) );
    }

    @Test
    public void chooseMinimal3() {
        assertEquals( 0, choose( 1, 2 ) );
    }

    @Test
    public void chooseMinimal4() {
        assertEquals( 10, choose( 5, 3 ) );
    }

    @Test
    public void chooseError1() {
        assertEquals( 0, choose( 5, -3 ) );
    }

    @Test
    public void chooseError2() {
        assertEquals( 0, choose( 5, 13 ) );
    }

    @Test
    public void chooseTest1() {
        assertEquals( 2598960, choose( 52, 5 ) );
    }

    @Test
    public void chooseTest2() {
        assertEquals( choose( 52, 5 ), choose( 52, 52 - 5 ) );
    }

    @Test
    public void chooseBig() {
        assertEquals( 88004802264174740L, choose( 60, 27 ) );
    }

    @Test
    public void partitionSmall() {
        List<Integer> naturalPartitions = asList( 1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627, 792, 1002 );
        for ( int i = 1; i < naturalPartitions.size(); i++ ) {
            Set<Integer> constrains = rangeClosed( 1, i ).mapToObj( Integer::valueOf ).collect( toSet() );
            assertEquals( (long) naturalPartitions.get( i ), partition( i, constrains ) );
        }
    }

}
