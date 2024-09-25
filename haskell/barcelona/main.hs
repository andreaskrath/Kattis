-- Barcelona
-- https://open.kattis.com/problems/barcelona

main = do
  lineOne <- getLine
  lineTwo <- getLine
  let target = last (words lineOne)
  let list = words lineTwo
  putStrLn (solution list target 1)

solution list target counter =
  let current = head list
   in if current == target
        then solutionHelper counter
        else solution (tail list) target (counter + 1)

solutionHelper n
  | n == 1 = "fyrst"
  | n == 2 = "naestfyrst"
  | otherwise = show n ++ " fyrst"
