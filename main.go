package main

import "fmt"

func main() {
	var a, b int
	fmt.Scanf("%d %d", &a, &b)
	if a <= b {
		fmt.Println(0)
	} else {
		fmt.Println(1)
	}
}
