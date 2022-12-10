// The Amazing Human Cannonball
// https://open.kattis.com/problems/humancannonball2

package main

import (
	"fmt"
	"math"
)

func main() {
	var inputAmount int
	fmt.Scan(&inputAmount)

	var startVelocity, canonAngle, canonDistance, heightLower, HeightUpper float64
	gravityAcceleration := 9.81
	for i := 0; i < inputAmount; i++ {
		fmt.Scanf("%f %f %f %f %f", &startVelocity, &canonAngle, &canonDistance, &heightLower, &HeightUpper)
		canonAngle = canonAngle * math.Pi / 180                                                                      // converting angle to radians
		time := canonDistance / (startVelocity * math.Cos(canonAngle))                                               // just isolating t in the x(t) formula
		yPosition := (startVelocity * time * math.Sin(canonAngle)) - (0.5 * gravityAcceleration * math.Pow(time, 2)) // we dont care about xPosition

		if yPosition-1 > heightLower && yPosition+1 < HeightUpper {
			fmt.Println("Safe")
		} else {
			fmt.Println("Not Safe")
		}
	}
}
