package main

import (
	"fmt"
	"math"
)

func main() {
	var left, right int
	fmt.Scanf("%d %d", &left, &right)

	if left != right {
		fmt.Println("Odd", 2*(math.Max(float64(left), float64(right))))
	} else if left == right && left != 0 {
		fmt.Println("Even", 2*(math.Max(float64(left), float64(right))))
	} else {
		fmt.Println("Not a moose")
	}
}
