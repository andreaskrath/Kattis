package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	var line string
	scanner := bufio.NewScanner(os.Stdin)
	if scanner.Scan() {
		line = scanner.Text()
	}

	line = strings.ToLower(line)
	vovelCount := 0
	for i := 0; i < len(line); i++ {
		switch line[i] {
		case 'a':
			fallthrough
		case 'e':
			fallthrough
		case 'i':
			fallthrough
		case 'o':
			fallthrough
		case 'u':
			vovelCount++
		default:
		}
	}

	fmt.Println(vovelCount)
}
