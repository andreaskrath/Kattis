package main

import "fmt"

func main() {
	var sideLength, horCut, verCut int
	fmt.Scanf("%d %d %d", &sideLength, &horCut, &verCut)
	// verCut moves from left side
	// horCut moves from top
	// 1 | 3
	// --|--
	// 2 | 4
	quadrants := make(map[int]int)
	for i := 0; i < 4; i++ {
		if i < 2 {
			quadrants[i] += verCut
		} else {
			quadrants[i] += sideLength - verCut
		}

		if i%2 == 0 {
			quadrants[i] *= horCut
		} else {
			quadrants[i] *= sideLength - horCut
		}

		quadrants[i] *= 4 // height of cake
	}

	max := quadrants[0]
	for _, v := range quadrants {
		if v > max {
			max = v
		}
	}
	fmt.Println(max)
}
