package main

import "fmt"

func main() {
	var stoneAmount uint32

	fmt.Scan(&stoneAmount)

	if stoneAmount%2 == 0 {
		fmt.Println("Bob")
	} else {
		fmt.Println("Alice")
	}
}
