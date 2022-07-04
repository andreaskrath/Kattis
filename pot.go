package main

import (
	"fmt"
	"math"
	"strconv"
)

func main() {
	var inputAmount, summed int
	var input string

	fmt.Scan(&inputAmount)

	for i := 0; i < inputAmount; i++ {
		fmt.Scan(&input)
		runedInput := []rune(input)
		// basically rune slice typecasted to string --> int, specifying the range to be entire slice minus last element
		if tempInt, err := strconv.Atoi(string(runedInput[:len(runedInput)-1])); err == nil {
			// same as earlier, just opposite really, only converting and interested in ranging the slice to the very last element
			if temptIntTwo, errTwo := strconv.Atoi(string(runedInput[len(runedInput)-1:])); errTwo == nil {
				summed += int(math.Pow(float64(tempInt), float64(temptIntTwo)))
			}
		}
	}
	fmt.Println(summed)
}
