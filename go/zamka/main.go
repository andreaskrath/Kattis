// Zamka
// https://open.kattis.com/problems/zamka

package main

import (
	"fmt"
	"strconv"
)

func main() {
	var lower, upper, target int

	fmt.Scan(&lower)
	fmt.Scan(&upper)
	fmt.Scan(&target)

	fmt.Println(findMin(lower, upper, target))
	fmt.Println(findMax(lower, upper, target))
}

func findMax(lower, upper, target int) int {
	for i := upper; i > lower; i-- {
		if sumDigits(fmt.Sprint(i), target) {
			return i
		}
	}
	return lower
}

func findMin(lower, upper, target int) int {
	for i := lower; i < upper; i++ {
		if sumDigits(fmt.Sprint(i), target) {
			return i
		}
	}
	return upper
}

func sumDigits(input string, target int) bool {
	var sum int

	for i := 0; i < len(input); i++ {
		if tempInt, err := strconv.Atoi(string(input[i])); err == nil {
			sum += tempInt
		}
	}
	if sum == target {
		return true
	} else {
		return false
	}
}

// L, D, X
// determine min int N, such that L <= N <= D and the sum of its digits is X
// determine max int M, such that L <= M <= D and the sum of its digits is X

// INPUT
// first line = L
// second line = D
// third line = X
