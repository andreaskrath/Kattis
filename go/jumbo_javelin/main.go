// Jumbo Javelin
// https://open.kattis.com/problems/jumbojavelin

package main

import "fmt"

func main() {
	var beamAmount, beamLength, fusedBeam int

	fmt.Scanln(&beamAmount)
	for i := 0; i < beamAmount; i++ {
		fmt.Scanln(&beamLength)
		fusedBeam += beamLength
	}

	fusedBeam = fusedBeam - (beamAmount - 1)

	fmt.Println(fusedBeam)
}
