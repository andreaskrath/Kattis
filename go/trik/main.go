// Trik
// https://open.kattis.com/problems/trik

package main

import "fmt"

func main() {
	var input string
	fmt.Scan(&input)

	cups := [3]int{1, 0, 0}
	for i := 0; i < len(input); i++ {
		switch input[i] {
		case 'A':
			cups[0], cups[1] = cups[1], cups[0]
		case 'B':
			cups[1], cups[2] = cups[2], cups[1]
		case 'C':
			cups[0], cups[2] = cups[2], cups[0]
		}
	}

	for i, v := range cups {
		if v == 1 {
			fmt.Println(i + 1)
		}
	}
}
