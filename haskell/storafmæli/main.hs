-- Stórafmæli
-- https://open.kattis.com/problems/storafmaeli

main = do
  year <- getLine
  putStrLn (solution year)

solution year =
  let num = (read year :: Int)
   in if (num `mod` 10) == 0
        then "Jebb"
        else "Neibb"
