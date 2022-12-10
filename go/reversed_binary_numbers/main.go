// Reversed Binary Numbers
// https://open.kattis.com/problems/reversebinary

package main

import (
	"fmt"
	"strconv"
)

func main() {
	var input int64
	fmt.Scan(&input)
	binaryInput := strconv.FormatInt(input, 2) // converting from whatever base, to base 2

	var reverseBinary string
	for i := len(binaryInput) - 1; i >= 0; i-- {
		reverseBinary += string(binaryInput[i])
	}

	if output, err := strconv.ParseInt(reverseBinary, 2, 64); err == nil {
		fmt.Println(output) // converting from base 2 to base 10
	}
}
