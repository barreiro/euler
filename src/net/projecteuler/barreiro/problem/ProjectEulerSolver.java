/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static java.lang.String.format;
import static java.util.Arrays.stream;
import static java.util.stream.IntStream.rangeClosed;

/**
 * Abstract class for a solver for a particular euler problem.
 *
 * @author barreiro
 */
public abstract class ProjectEulerSolver {

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
     * Method that is implemented by each solver
     *
     * @return Solution for the problem
     */
    protected abstract long solve();

    /* --- */

    /**
     * Entry point. Solves problems.
     *
     * @param args Problem or problems to solve
     */
    public static void main(final String[] args) {
        if (args.length == 0) {
            solveAll();
        } else {
            stream(args).mapToInt(Integer::valueOf).forEachOrdered(ProjectEulerSolver::solve);
        }
    }

    /* --- */

    /**
     * Solves all problems! The Euler ones.
     *
     */
    private static void solveAll() {
        rangeClosed(1, 20).forEachOrdered(ProjectEulerSolver::solve);
    }

    /**
     * Solves a particular Euler problem.
     *
     * @param number The number of the problem to solve
     */
    private static void solve(int number) {
        try {
            String solverClassName = format("%s%s%03d", ProjectEulerSolver.class.getPackage().getName(), ".Solver", number);
            ProjectEulerSolver solverInstance = ProjectEulerSolver.class.cast(Class.forName(solverClassName).newInstance());
            if (solverInstance != null) {
                if (Boolean.valueOf(System.getProperty("euler.traceExecutionTime", "false"))) {
                    long start = System.currentTimeMillis();
                    System.out.printf("Solution for problem %03d is %d --- Took %d ms%n", number, solverInstance.solve(), System.currentTimeMillis() - start);
                } else {
                    System.out.printf("Solution for problem %03d is %d%n", number, solverInstance.solve());
                }
            }
        } catch (ClassNotFoundException e) {
            System.out.printf("ERROR: No implementation found for problem %d%n", number);
        } catch (Exception e) {
            System.out.printf("ERROR: Exception during execution of problem %s: %s%n", number, e.getMessage());
        }
    }

}
