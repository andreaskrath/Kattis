// Bela
// https://open.kattis.com/problems/bela

package main

import "fmt"

func main() {
	var hands int
	var dominantSuit string

	fmt.Scanf("%d %s", &hands, &dominantSuit)

	var totalPoints int
	var tempStr string
	for i := 0; i < 4*hands; i++ {
		fmt.Scan(&tempStr)
		if string(tempStr[1]) == dominantSuit {
			switch rune(tempStr[0]) {
			case 'A':
				totalPoints += 11
			case 'K':
				totalPoints += 4
			case 'Q':
				totalPoints += 3
			case 'J':
				totalPoints += 20
			case 'T':
				totalPoints += 10
			case '9':
				totalPoints += 14
			default:
			}
		} else {
			switch rune(tempStr[0]) {
			case 'A':
				totalPoints += 11
			case 'K':
				totalPoints += 4
			case 'Q':
				totalPoints += 3
			case 'J':
				totalPoints += 2
			case 'T':
				totalPoints += 10
			default:
			}
		}
	}

	fmt.Println(totalPoints)
}
