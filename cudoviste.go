package main

import "fmt"

type Coordinates struct {
	x int
	y int
}

func main() {
	var inputRows, inputColumns int
	fmt.Scanf("%d %d", &inputRows, inputColumns)

	var inputGrid []string
	var tempStr string
	for i := 0; i < inputRows; i++ {
		fmt.Println(i)
		fmt.Scanln(&tempStr)
		inputGrid = append(inputGrid, tempStr)
	}
	fmt.Println("hi")
	var parkingSpots []int
	var position Coordinates

	// continue expression ensures only proper function calls are made
	// so no out of bounds
	for i := 0; i < inputRows-1; i++ {
		for j := 0; j < inputColumns-1; j++ {
			position.x = i
			position.y = j
			parkingSpots[checkGrid(inputGrid, position)]++
		}
	}

	for _, v := range parkingSpots {
		fmt.Println(v)
	}
}

func checkGrid(grid []string, position Coordinates) int {
	fmt.Println("hi2")
	var cars int
	// rows
	for i := 0; i < 2; i++ {
		// columns
		for j := 0; i < 2; j++ {
			switch string(grid[i][j]) {
			case "#": // building
				return 0
			case "X": // parked car
				cars++
			default:
			}
		}
	}
	return cars
}
