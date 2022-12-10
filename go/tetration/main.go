// Tetration
// https://open.kattis.com/problems/tetration

package main

import (
	"fmt"
	"math"
)

func main() {
	var input float64
	fmt.Scan(&input)
	fmt.Println(math.Pow(input, (1.0 / input)))
}
