package main

import "fmt"

func main() {
	var width, pieces int

	fmt.Scan(&width)
	fmt.Scan(&pieces)

	var tempW, tempL, area int
	for i := 0; i < pieces; i++ {
		fmt.Scanf("%d %d", &tempW, &tempL)
		area += tempL * tempW
	}
	fmt.Println(area / width)
}
