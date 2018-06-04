// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems

package util

import "runtime"

const defaultBuffer = 500

func ParallelUnbound(lowerBound int, supplier func(int) int, predicate func(int) bool) int {
	source, result := make(chan int, defaultBuffer), make(chan int)
	startWorkers(source, result, predicate)
	for n := lowerBound; ; n++ {
		select {
		case source <- supplier(n):
		case r := <-result:
			close(source)
			return r
		}
	}
}

func startWorkers(source, result chan int, predicate func(int) bool) {
	for i := 0; i < runtime.NumCPU(); i++ {
		go func() {
			for n, ok := 0, true; ok; {
				if n, ok = <-source; predicate(n) {
					result <- n
				}
			}
		}()
	}
}
