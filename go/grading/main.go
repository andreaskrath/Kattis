// Grading
// https://open.kattis.com/problems/grading

package main

import "fmt"

func main() {
	var a, b, c, d, e, grade int
	if _, err := fmt.Scanf("%d %d %d %d %d", &a, &b, &c, &d, &e); err != nil {
		fmt.Println(err)
	}
	fmt.Scan(&grade)

	switch {
	case grade >= a:
		fmt.Println("A")
	case grade >= b:
		fmt.Println("B")
	case grade >= c:
		fmt.Println("C")
	case grade >= d:
		fmt.Println("D")
	case grade >= e:
		fmt.Println("E")
	default:
		fmt.Println("F")
	}
}
