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

	var output []string
	var temp string

	for scanner.Scan() {
		matrixSize, err := strconv.Atoi(scanner.Text())
		if err != nil {
			return
		}
		if matrixSize == -1 {
			for _, v := range output {
				fmt.Println(v)
			}
			return
		}

		m := make([][]int, matrixSize)
		for i := range m {
			m[i] = make([]int, matrixSize)
		}

		// scan matrix
		for i := 0; i < matrixSize; i++ {
			for j := 0; j < matrixSize; j++ {
				fmt.Scan(&m[i][j])
			}
		}

		// each node
		for i := 0; i < matrixSize; i++ {
			weak := true
			// direct connections
			for j := 0; j < matrixSize; j++ {
				// nested connections
				for k := 0; k < matrixSize; k++ {
					if m[i][k] == 1 && m[i][j] == 1 && m[j][k] == 1 && i != j && i != k && j != k {
						weak = false
					}
				}
			}
			if weak {
				temp += fmt.Sprint(i, " ")
			}
		}
		output = append(output, temp)
		temp = ""
	}
}
