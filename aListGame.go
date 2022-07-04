package main

import (
	"fmt"
	"math"
)

func main() {
	var num int
	fmt.Scan(&num)
	i := 0
	for {
		if math.Pow(float64(i+1), float64(i)) == float64(num) {
			fmt.Println(i)
			break
		}
		i++
	}
}
