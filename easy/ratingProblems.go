package main

import "fmt"

func main() {
	var totalJudges, currentJudges, tempInt, totalPoints int
	var judgeScores []int

	fmt.Scanf("%d %d", &totalJudges, &currentJudges)

	for i := 0; i < currentJudges; i++ {
		fmt.Scan(&tempInt)
		judgeScores = append(judgeScores, tempInt)
	}

	if totalJudges == currentJudges {
		for _, value := range judgeScores {
			totalPoints += value
		}
		fmt.Println(float32(totalPoints)/float32(totalJudges), float32(totalPoints)/float32(totalJudges))
	} else {
		for _, value := range judgeScores {
			totalPoints += value
		}
		maxPoints := totalPoints + (3 * (totalJudges - currentJudges))
		minPoints := totalPoints + (-3 * (totalJudges - currentJudges))
		fmt.Println(float32(minPoints)/float32(totalJudges), float32(maxPoints)/float32(totalJudges))
	}
}
