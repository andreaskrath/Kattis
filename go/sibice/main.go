// Sibice
// https://open.kattis.com/problems/sibice

package main

import (
	"fmt"
	"math"
)

func main() {
	var matches, height, width, tempInt int

	var matchLengths []int

	fmt.Scanf("%d %d %d", &matches, &width, &height)

	boxDiagonal := math.Sqrt(math.Pow(float64(height), 2) + math.Pow(float64(width), 2))

	for i := 0; i < matches; i++ {
		fmt.Scan(&tempInt)
		matchLengths = append(matchLengths, tempInt)
	}
	for _, value := range matchLengths {
		if float64(value) > boxDiagonal {
			fmt.Println("NE")
		} else {
			fmt.Println("DA")
		}
	}
}
