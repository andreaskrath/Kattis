// Number Fun
// https://open.kattis.com/problems/numberfun
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	var cases int
	fmt.Scan(&cases)

	scanner := bufio.NewScanner(os.Stdin)

	var a, b, c int
	var err error
	for i := 0; i < cases; i++ {
		scanner.Scan()
		input := scanner.Text()
		splitInput := strings.Split(input, " ")

		if a, err = strconv.Atoi(splitInput[0]); err != nil {
			panic(err)
		}
		if b, err = strconv.Atoi(splitInput[1]); err != nil {
			panic(err)
		}
		if c, err = strconv.Atoi(splitInput[2]); err != nil {
			panic(err)
		}

		if possibility(a, b, c) {
			fmt.Println("Possible")
		} else {
			fmt.Println("Impossible")
		}
	}
}

func possibility(a, b, c int) bool {
	//only division and subtraction are not associative, so two cases are needed
	if (a + b) == c {
		return true
	}
	if (a - b) == c {
		return true
	}
	if (b - a) == c {
		return true
	}
	if (float32(a) / float32(b)) == float32(c) {
		return true
	}
	if (float32(b) / float32(a)) == float32(c) {
		return true
	}
	if (a * b) == c {
		return true
	}

	return false
}
