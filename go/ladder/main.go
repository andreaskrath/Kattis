// Ladder
// https://open.kattis.com/problems/ladder

package main

import (
	"fmt"
	"math"
)

func main() {
	var height, angle float64
	fmt.Scanf("%f %f", &height, &angle)
	fmt.Println(math.Ceil((height * math.Sin(90*(math.Pi/180))) / math.Sin(angle*(math.Pi/180)))) //sinun relation
}
