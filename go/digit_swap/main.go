// Digit Swap
// https://open.kattis.com/problems/digitswap

package main

import "fmt"

func main() {
	var input string

	fmt.Scan(&input)

	runedInput := []rune(input)

	runedInput[0], runedInput[1] = runedInput[1], runedInput[0]

	fmt.Println(string(runedInput))
}
