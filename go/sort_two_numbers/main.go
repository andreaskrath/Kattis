// Sort Two Numbers
// https://open.kattis.com/problems/sorttwonumbers

package main

import "fmt"

func main() {
	var a, b int

	fmt.Scanf("%d %d", &a, &b)

	if a < b {
		fmt.Println(a, b)
	} else {
		fmt.Println(b, a)
	}
}
