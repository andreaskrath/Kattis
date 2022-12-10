// Cetvrta
// https://open.kattis.com/problems/cetvrta

package main

import "fmt"

func main() {
	var tempX, tempY int
	xMap := make(map[int]int)
	yMap := make(map[int]int)

	for i := 0; i < 3; i++ {
		fmt.Scanf("%d %d", &tempX, &tempY)
		xMap[tempX]++
		yMap[tempY]++
	}

	var newXCoord int
	for key, element := range xMap {
		if element == 1 {
			newXCoord = key
		}
	}
	var newYCoord int
	for key, element := range yMap {
		if element == 1 {
			newYCoord = key
		}
	}

	fmt.Println(newXCoord, newYCoord)
}
