// N-sum
// https://open.kattis.com/problems/nsum
package main

import "fmt"

func main() {
	var inputAmount, tempInt, outputNumber int

	fmt.Scan(&inputAmount)

	for i := 0; i < inputAmount; i++ {
		fmt.Scan(&tempInt)
		outputNumber += tempInt
	}

	fmt.Println(outputNumber)
}
