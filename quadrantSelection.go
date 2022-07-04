package main

import "fmt"

func main() {
	var xCoord, yCoord int16

	fmt.Scan(&xCoord)
	fmt.Scan(&yCoord)

	if xCoord > 0 {
		if yCoord > 0 {
			fmt.Print(1)
		} else {
			fmt.Print(4)
		}
	} else {
		if yCoord > 0 {
			fmt.Print(2)
		} else {
			fmt.Print(3)
		}
	}
}
