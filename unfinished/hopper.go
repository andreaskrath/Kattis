package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"sync"
)

var wg sync.WaitGroup

func main() {
	var arrLength, maxJumpDist, maxValDiff int
	fmt.Scanf("%d %d %d", &arrLength, &maxJumpDist, &maxValDiff)
	var arr []int
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)

	for i := 0; i < arrLength; i++ {
		scanner.Scan()
		arrStr := scanner.Text()
		tempInt, err := strconv.Atoi(arrStr)
		if err != nil {
			panic(err)
		}
		arr = append(arr, tempInt)
	}

	depthChan := make(chan int, arrLength)
	wg.Add(arrLength)
	for i := 0; i < arrLength; i++ {
		visited := make(map[int]bool)
		visited[i] = true
		go func(index int) {
			defer wg.Done()
			depthChan <- deepDive(arr, visited, maxJumpDist, maxValDiff, index)
		}(i)
	}

	wg.Wait()
	close(depthChan)
	var n int
	for v := range depthChan {
		if n < v {
			n = v
		}
	}
	fmt.Println(n)
}

func deepDive(arr []int, visited map[int]bool, maxJumpDist, maxValDiff, index int) int {
	depth := len(visited)

	// optimize by making a more efficient loop, only looping within max jump distance
	// as currently, a lot of useless comparisons are made
	// consider how to handle prior negative index from i < index

	for i, v := range arr {
		if visited[i] || abs(index-i) > maxJumpDist || abs(arr[index]-v) > maxValDiff {
			continue
		}
		visitedCopy := visited
		visitedCopy[i] = true
		tempInt := deepDive(arr, visitedCopy, maxJumpDist, maxValDiff, i)
		if tempInt > depth {
			depth = tempInt
		}
	}
	return depth
}

func abs(n int) int {
	if n < 0 {
		n *= -1
	}
	return n
}
