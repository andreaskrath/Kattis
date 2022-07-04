package main

import (
	"fmt"
	"math"
)

func main() {
	var a, b, c, d float64
	fmt.Scanf("%f %f %f %f", &a, &b, &c, &d)
	s := (a + b + c + d) / 2.0
	fmt.Println(math.Sqrt((s - a) * (s - b) * (s - c) * (s - d))) // Brahmagupta's or Bretschneider's formula
}
