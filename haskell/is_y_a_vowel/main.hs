-- Is Y a Vowel?
-- https://open.kattis.com/problems/isyavowel

main = do
  word <- getLine
  let (count, yCount) = solution word 0 0
  putStrLn (show count ++ " " ++ show yCount)

solution "" count yCount = (count, yCount)
solution word count yCount =
  let current = head word
   in if current == 'a' || current == 'e' || current == 'i' || current == 'o' || current == 'u'
        then solution newWord (count + 1) (yCount + 1)
        else
          if current == 'y'
            then solution newWord count (yCount + 1)
            else solution newWord count yCount
  where
    newWord = tail word
