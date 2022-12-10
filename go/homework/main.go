package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	var input string

	fmt.Scanln(&input)

	splitInput := strings.Split(input, ";") // splitting original input format (1-3;5;6;8-10)

	var homeworkCount, sliceCounter int
	var rangeHomework []int
	for _, value := range splitInput { // ranging over the amount of instances to be computed
		if strings.Contains(value, "-") == true { // if exercise is a range instead of single task
			tempSlice := strings.Split(value, "-")

			for _, valueTwo := range tempSlice {
				if tempInt, err := strconv.Atoi(valueTwo); err == nil { // have to check if err or not
					rangeHomework = append(rangeHomework, tempInt)
				}
			}
			homeworkCount += 1 + rangeHomework[sliceCounter+1] - rangeHomework[sliceCounter] // higher number - lower number + 1, for correct counting
			sliceCounter += 2                                                                // either we reset content of rangeHomework and increment counter, or simply increment counter (of course more memory usage)
		} else {
			homeworkCount++ // single task
		}
	}

	fmt.Println(homeworkCount)
}
