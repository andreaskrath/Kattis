package main

import "fmt"

func main() {
	var a, b int
	var tempString string

	fmt.Scanf("%d %d", &a, &b)

	for i := 0; i < a; i++ {
		fmt.Scanln(&tempString)
	}
	fmt.Print(b)
}
