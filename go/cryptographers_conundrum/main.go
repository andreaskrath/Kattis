// Cryptographer's Conundrum
// https://open.kattis.com/problems/conundrum

package main

import "fmt"

func main() {
	var s string
	fmt.Scan(&s)
	factor := len(s) / 3

	var counter int
	for i := 0; i < factor; i++ {
		if s[0+(3*i)] == byte('P') {
			counter++
		}
		if s[1+(3*i)] == byte('E') {
			counter++
		}
		if s[2+(3*i)] == byte('R') {
			counter++
		}
	}

	fmt.Println(len(s) - counter)
}

// for i, v := range s {
// 	switch i % 3{
// 	case 0:
// 		if v == 'P' {
// 			counter++
// 		}
// 	case 1:
// 		if v == 'E' {
// 			counter++
// 		}
// 	case 2:
// 		if v == 'R' {
// 			counter++
// 		}
// 	}
// }
