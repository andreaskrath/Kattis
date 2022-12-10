// Filip
// https://open.kattis.com/problems/filip

package main

import (
	"fmt"
	"math"
	"strconv"
)

func main() {
	var numOne, numTwo string

	fmt.Scanf("%s %s", &numOne, &numTwo)

	revNumOneStr := reverse(numOne)
	revNumTwoStr := reverse(numTwo)

	revNumOneInt, err := strconv.Atoi(revNumOneStr)
	if err != nil {
		fmt.Println(err)
		return
	}
	revNumTwoInt, errTwo := strconv.Atoi(revNumTwoStr)
	if errTwo != nil {
		fmt.Println(errTwo)
		return
	}

	fmt.Println(math.Max(float64(revNumOneInt), float64(revNumTwoInt)))
}

func reverse(input string) string {
	runedInput := []rune(input)

	for i, j := 0, len(input)-1; i <= j; i, j = i+1, j-1 {
		runedInput[i], runedInput[j] = runedInput[j], runedInput[i]
	}

	return string(runedInput)
}
