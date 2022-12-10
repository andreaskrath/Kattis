// Synchronizing Lists
// https://open.kattis.com/problems/synchronizinglists

package main

import (
	"fmt"
	"sort"
)

func main() {
	var listSize int
	for {
		fmt.Scanln(&listSize)
		if listSize == 0 {
			return
		}
		newList := orderList(scanList(listSize), scanList(listSize))
		for _, v := range newList {
			fmt.Println(v)
		}
		fmt.Println("")
	}
}

func scanList(n int) []int {
	var list []int
	var temp int
	for i := 0; i < n; i++ {
		fmt.Scanln(&temp)
		list = append(list, temp)
	}
	return list
}

func orderList(one, two []int) []int {
	posMap := make(map[int]int)
	for i, v := range one {
		posMap[v] = i
	}
	sort.Ints(one)
	sort.Ints(two)
	newList := make([]int, len(one))
	for i, v := range two {
		newList[posMap[one[i]]] = v
	}
	return newList
}
