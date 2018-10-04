// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems

package util

import (
	"runtime"
	"sync"
)

const defaultBuffer = 100

func ParallelFill(array []int, function func(int) int) {
	source, wg := make(chan int, defaultBuffer), sync.WaitGroup{}
	for i := 0; i < runtime.NumCPU(); i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for n, ok := 0, true; ok; {
				if n, ok = <-source; ok {
					array[n] = function(n)
				}
			}
		}()
	}
	for i := range array {
		source <- i
	}
	close(source)
	wg.Wait()
}

func ParallelUnbound(floor int, supplier func(int) int, predicate func(int) bool) int {
	source, result := make(chan int, defaultBuffer), make(chan int)
	for i := 0; i < runtime.NumCPU(); i++ {
		go func() {
			for n, ok := 0, true; ok; {
				if n, ok = <-source; predicate(n) {
					result <- n
				}
			}
		}()
	}
	for n := floor; ; n++ {
		select {
		case source <- supplier(n):
		case r := <-result:
			close(source)
			return r
		}
	}
}
