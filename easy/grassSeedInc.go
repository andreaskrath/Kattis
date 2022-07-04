package main

import "fmt"

func main() {
	var cost, width, height, totalCost float32
	var lawnAmount int

	fmt.Scan(&cost)
	fmt.Scan(&lawnAmount)

	for i := 0; i < lawnAmount; i++ {
		fmt.Scanf("%f %f", &width, &height)
		area := width * height
		totalCost += area * cost
	}

	fmt.Println(totalCost)
}
