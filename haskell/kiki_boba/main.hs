-- Kiki Boba
-- https://open.kattis.com/problems/kikiboba

main = do
  word <- getLine
  putStrLn (solution word 0 0)

solution [] kCount bCount
  | kCount > bCount = "kiki"
  | kCount < bCount = "boba"
  | kCount == bCount && kCount /= 0 = "boki"
  | otherwise = "none"
solution word kCount bCount =
  let current = head word
   in if current == 'k'
        then solution newWord (kCount + 1) bCount
        else
          if current == 'b'
            then solution newWord kCount (bCount + 1)
            else solution newWord kCount bCount
  where
    newWord = tail word
