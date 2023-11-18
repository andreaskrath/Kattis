// Á leið í bíó
// https://open.kattis.com/problems/aleidibio

package main

import "fmt"

func main() {
	var a, b, c int
	fmt.Scanf("%d", &a)
	fmt.Scanf("%d", &b)
	fmt.Scanf("%d", &c)

	res := solution(a, b, c)
	fmt.Println(res)
}

func solution(a, b, c int) int {
	return c - b - a
}
