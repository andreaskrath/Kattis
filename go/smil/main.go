// SMIL
// https://open.kattis.com/problems/smil

package main

import "fmt"

func main() {
	var input string
	var smileys []int

	fmt.Scan(&input)

	for i := 0; i < len(input); i++ {
		if input[i] == ':' || input[i] == ';' {
			if i < len(input)-1 && input[i+1] == ')' { // important to check length first, to not go out of bounds
				smileys = append(smileys, i)
			}
			if i < len(input)-2 && input[i+1] == '-' && input[i+2] == ')' {
				smileys = append(smileys, i)
			}
		}
	}

	for _, v := range smileys {
		fmt.Println(v)
	}
}
