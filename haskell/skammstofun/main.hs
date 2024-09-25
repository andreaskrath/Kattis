-- Skammstöfun
-- https://open.kattis.com/problems/skammstofun

main = do
  _ <- getLine
  line <- getLine
  putStrLn (solution (words line) [])

solution [] abbreviation = abbreviation
solution (current : list) abbreviation =
  let currentChar = head current
   in if currentChar `elem` ['A' .. 'Z']
        then solution list (abbreviation ++ [currentChar])
        else solution list abbreviation
