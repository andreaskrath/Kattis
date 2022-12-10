// Turn It Up!
// https://open.kattis.com/problems/skruop

package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var requestNum int
	fmt.Scan(&requestNum)

	scanner := bufio.NewScanner(os.Stdin)
	volumeLevel := 7

	for i := 0; i < requestNum; i++ {
		scanner.Scan()
		if volumeLevel != 10 && scanner.Text() == "Skru op!" {
			volumeLevel++
		}
		if volumeLevel != 0 && scanner.Text() == "Skru ned!" {
			volumeLevel--
		}
	}
	fmt.Println(volumeLevel)
}
