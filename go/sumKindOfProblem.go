package main

import "fmt"

func main() {
	var cases int
	fmt.Scan(&cases)

	for i := 0; i < cases; i++ {
		dataNr, caseNr := 0, 0
		fmt.Scanf("%d %d", &dataNr, &caseNr)

		sumNPosInt := 0
		for i := 1; i <= caseNr; i++ {
			sumNPosInt += i
		}

		sumNOddInt := 0
		for i, j := 1, 1; j <= caseNr; i++ {
			if i%2 == 1 {
				sumNOddInt += i
				j++
			}
		}

		sumNEvenInt := 0
		for i, j := 1, 1; j <= caseNr; i++ {
			if i%2 == 0 {
				sumNEvenInt += i
				j++
			}
		}

		fmt.Println(dataNr, sumNPosInt, sumNOddInt, sumNEvenInt)
	}
}
