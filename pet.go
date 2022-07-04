package main

import "fmt"

func main() {
	var currentLeadPoints, currentLeadPosition, tempInt int
	var points []int

	for i := 0; i < 5; i++ {
		for j := 0; j < 4; j++ {
			fmt.Scan(&tempInt)
			points = append(points, tempInt)
		}
		tempInt = 0
		for _, value := range points {
			tempInt += value
		}
		if tempInt > currentLeadPoints {
			currentLeadPoints = tempInt
			currentLeadPosition = i + 1
		}
		tempInt = 0
		points = points[:0] // basically assigning it to cover no of the current elements, effetively = nil, but it keeps the underlying array
	}

	fmt.Println(currentLeadPosition, currentLeadPoints)

}
