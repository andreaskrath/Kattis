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
	for i := 0; i < cases; i++ {
		scanner.Scan()
		s := scanner.Text()
		root := int(math.Sqrt(float64(len(s))))
		subStr := make(map[int]string)

		for i, v := range s {
			subStr[(i % root)] += string(v)
		}
		output := ""
		for j := root - 1; j >= 0; j-- {
			output += subStr[j]
		}
		fmt.Println(output)
	}
}
