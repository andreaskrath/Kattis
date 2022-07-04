package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
)

func main() {
	var cases int
	fmt.Scan(&cases)

	scanner := bufio.NewScanner(os.Stdin)
	var output string
	for i := 0; i < cases; i++ {
		scanner.Scan()
		s := scanner.Text()
		root := int(math.Sqrt(float64(len(s))))

		for sqrIndex := root - 1; sqrIndex >= 0; sqrIndex-- {
			for j, curIndex := 0, sqrIndex; j < root; j++ {
				output += string(s[curIndex])
				curIndex += root
			}
		}
		fmt.Println(output)
		output = ""
	}
}
