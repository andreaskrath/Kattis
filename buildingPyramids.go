package main

import "fmt"

func main() {
	var blockNum int
	fmt.Scan(&blockNum)
	fmt.Println(findHeight(blockNum))
}

func findHeight(number int) int {
	curBlocks, i, height := 0, 1, 0
	for true {
		curBlocks += i * i

		if curBlocks > number {
			return height
		}

		i += 2
		height++
	}

	return height
}
