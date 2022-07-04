package main

import (
	"fmt"
	"math"
	"sort"
)

func main() {
	var cases int
	fmt.Scan(&cases)

	for i := 0; i < cases; i++ {
		var parkingSpots int
		fmt.Scan(&parkingSpots)

		temp, distance := 0, 0
		locations := []int{}
		for j := 0; j < parkingSpots; j++ {
			fmt.Scan(&temp)
			locations = append(locations, temp)
		}
		if len(locations) == 1 {
			fmt.Println(0)
		} else if len(locations) == 2 {
			fmt.Println(2 * locations[0])
		} else {
			sort.Ints(locations)
			mid := int(math.Floor(float64(len(locations))/2.0)) - 1
			locations = append(locations[:mid], locations[mid+1:]...)
			// start and end
			distance += (mid - locations[0]) + (locations[len(locations)-1] - mid)

			for i := 0; i < len(locations)-1; i++ {
				distance += locations[i+1] - locations[i]
			}
			fmt.Println(distance)
		}
	}
}
