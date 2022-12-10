// R2
// https://open.kattis.com/problems/r2

package main

import "fmt"

func main() {
	var rOne, s float32

	fmt.Scanf("%f %f", &rOne, &s)

	programRun := true
	if rOne < s {
		for i := s; programRun != false; i++ {
			if (rOne+i)/2 == s {
				fmt.Println(i)
				programRun = false
			}
		}
	} else {
		for i := rOne; programRun != false; i-- {
			if (rOne+i)/2 == s {
				fmt.Println(i)
				programRun = false
			}
		}
	}
}
