// FYI
// https://open.kattis.com/problems/fyi

package main

import (
	"fmt"
	"strings"
)

func main() {
	var input string

	fmt.Scan(&input)

	if strings.HasPrefix(input, "555") {
		fmt.Println(1)
	} else {
		fmt.Println(0)
	}
}
