/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static java.lang.Boolean.parseBoolean;
import static java.lang.Class.forName;
import static java.lang.String.format;
import static java.lang.System.currentTimeMillis;
import static java.lang.System.gc;
import static java.lang.System.getProperty;
import static java.lang.System.out;
import static java.lang.Thread.sleep;
import static java.util.stream.IntStream.rangeClosed;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.intStream;

/**
 * Abstract class for a solver for a particular euler problem.
 *
 * @author barreiro
 */
public abstract class ProjectEulerSolver {

    private static final int SOLVED_SO_FAR = 40;

    /**
     * Scale of the problem.
     */
    protected final long N;

    /**
     * It's very useful that for every problem solver we define some sort of dimension. Mostly for testing purposes.
     *
     * @param n Scale of the problem
     */
    protected ProjectEulerSolver(long n) {
        this.N = n;
    }

    /**
     * Entry point. Solves problems.
     *
     * @param args Problem or problems to solve
     */
    public static void main(String... args) {
        if ( args.length == 0 ) {
            solveAll();
        } else {
            intStream( args ).forEachOrdered( ProjectEulerSolver::solveSingle );
        }
    }

    // --- //

    /**
     * Solves all problems! The Euler ones.
     */
    private static void solveAll() {
        rangeClosed( 1, SOLVED_SO_FAR ).forEachOrdered( ProjectEulerSolver::solveSingle );
    }

    // --- //

    /**
     * Solves a particular Euler problem.
     *
     * @param number The number of the problem to solve
     */
    private static void solveSingle(int number) {
        try {
            String solverClassName = format( "%s.Solver%03d", ProjectEulerSolver.class.getPackage().getName(), number );
            ProjectEulerSolver solverInstance = ProjectEulerSolver.class.cast( forName( solverClassName ).newInstance() );
            if ( solverInstance != null ) {
                if ( parseBoolean( getProperty( "euler.traceExecutionTime", "false" ) ) ) {
                    gc();
                    sleep( 100 );
                    long start = currentTimeMillis();
                    info( "Solution for problem %03d is %12d ( %3d ms )", number, solverInstance.solve(), currentTimeMillis() - start );
                } else {
                    info( "Solution for problem %03d is %12d", number, solverInstance.solve() );
                }
            }
        } catch ( ClassNotFoundException e ) {
            warn( "ERROR: No implementation found for problem %d", number );
        } catch ( Exception e ) {
            warn( "ERROR: Exception during execution of problem %d: %s", number, e );
        }
    }

    private static void info(String msg, Object... args) {
        out.println( format( msg, args ) );
    }

    private static void warn(String msg, Object... args) {
        out.println( format( msg, args ) );
    }

    /**
     * Method that is implemented by each solver
     *
     * @return Solution for the problem
     */
    protected abstract long solve();
}
