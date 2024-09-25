-- Fjöldi Bókstafa
-- https://open.kattis.com/problems/fjoldibokstafa

main = do
  line <- getLine
  print (length (filter isMyLetter line))

isMyLetter c = c `elem` ['a' .. 'z'] || c `elem` ['A' .. 'Z']
