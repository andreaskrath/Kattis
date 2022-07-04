package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)

	scanner.Scan()
	numOfCompanies := convertInt(scanner.Text())
	scanner.Scan()
	inputAmount := convertInt(scanner.Text())

	companyLocation := make(map[int]int)

	for i := 1; i <= numOfCompanies; i++ {
		scanner.Scan()
		initialLoc := convertInt(scanner.Text())
		companyLocation[i] += initialLoc
	}

	for i := 0; i < inputAmount; i++ {
		scanner.Scan()
		a := convertInt(scanner.Text())
		scanner.Scan()
		b := convertInt(scanner.Text())
		scanner.Scan()
		c := convertInt(scanner.Text())
		if a == 1 {
			companyLocation[b] = c
		} else {
			distance := companyLocation[b] - companyLocation[c]
			if distance < 0 {
				fmt.Println(-1 * distance)
			} else {
				fmt.Println(distance)
			}
		}
	}
}

func convertInt(a string) int {
	output, err := strconv.Atoi(a)
	if err != nil {
		return 0
	}
	return output
}
