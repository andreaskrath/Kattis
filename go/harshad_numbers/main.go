// Harshad Numbers
// https://open.kattis.com/problems/harshadnumbers

package main

import (
	"fmt"
	"strconv"
)

func main() {
	var input string

	fmt.Scan(&input)

	fmt.Println(findHarshad(input))
}

func sumDigits(input string) int {
	var sum int

	for i := 0; i < len(input); i++ {
		if tempInt, err := strconv.Atoi(string(input[i])); err == nil {
			sum += tempInt
		}
	}

	return sum
}

func findHarshad(input string) int {
	var inputInt int
	var err error
	if inputInt, err = strconv.Atoi(input); err != nil {
		fmt.Println(err)
	}

	for i := inputInt; ; i++ {
		if i%sumDigits(fmt.Sprint(i)) == 0 {
			return i
		}
	}
}
