package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	var teamAmount int
	fmt.Scan(&teamAmount)

	teams := make(map[string]int)
	scanner := bufio.NewScanner(os.Stdin)

	for i := 0; i < 12; {
		scanner.Scan()
		s := strings.Split(scanner.Text(), " ")
		_, exist := teams[s[0]]
		if exist {
			continue
		} else {
			teams[s[0]] = 1
			i++
			fmt.Println(s[0] + " " + s[1])
		}
	}
}
