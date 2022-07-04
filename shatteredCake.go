package main

import (
	"fmt"
	"sync"
)

var waitList sync.WaitGroup

type CakePiece struct {
	width  int
	length int
}

func main() {
	var cWidth, cPieces, area int
	fmt.Scan(&cWidth)
	fmt.Scan(&cPieces)

	areaChannel := make(chan int)

	for i := 0; i < cPieces; i++ {
		waitList.Add(1)
		go findArea(areaChannel)
		area += <-areaChannel
	}
	waitList.Wait()

	fmt.Println(area / cWidth)
}

func findArea(areaC chan int) {
	defer waitList.Done()
	var tempW, tempL int

	fmt.Scan(&tempW)
	fmt.Scan(&tempL)
	areaC <- tempW * tempL
}

// doesnt make time limit cause fmt.Scan is slow af
