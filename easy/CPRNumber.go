package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	constWeights := []int{4, 3, 2, 7, 6, 5, 4, 3, 2, 1} // pre-determined
	var s string
	fmt.Scan(&s)

	s2 := strings.Split(s, "-")
	s3 := s2[0] + s2[1]

	var sum int
	for i, v := range s3 {
		sum += constWeights[i] * convertInt(string(v))
	}
	if sum%11 == 0 {
		fmt.Println(1)
	} else {
		fmt.Println(0)
	}
}

func convertInt(s string) int {
	n, err := strconv.Atoi(s)
	if err != nil {
		os.Exit(0)
	}
	return n
}
