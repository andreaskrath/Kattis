package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	var lines int
	fmt.Scan(&lines)

	scanner := bufio.NewScanner(os.Stdin)
	var nums []int
	for i := 0; i < lines; i++ {
		scanner.Scan()
		nums = append(nums, convertInt(scanner.Text()))
	}

	for i := len(nums) - 1; i >= 0; i-- {
		fmt.Println(nums[i])
	}
}

func convertInt(s string) int {
	n, err := strconv.Atoi(s)
	if err != nil {
		return 0
	}
	return n
}
