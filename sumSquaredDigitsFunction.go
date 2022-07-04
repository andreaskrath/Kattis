package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)

	var s string
	var counter, result, base int
	for i := 0; i < n; i++ {
		fmt.Scanf("%d %d %s", &counter, &base, &s)
		for _, v := range s {
			result += int(v)
		}
	}
}
