-- Cosmic Path Optimization
-- https://open.kattis.com/problems/cosmicpathoptimization

main = do
  amountString <- getLine
  let amount = read amountString :: Float
  line <- getLine
  print (floor (fromIntegral (sum (map read (words line) :: [Int])) / amount))
