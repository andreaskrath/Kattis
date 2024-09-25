-- Skák
-- https://open.kattis.com/problems/skak

main = do
  petraRook <- getLine
  gardarPawn <- getLine
  putStrLn (solution (words petraRook) (words gardarPawn))

solution (px : py) (gx : gy) =
  if px == gx || py == gy
    then "1"
    else "2"
