-- Takk fyrir mig
-- https://open.kattis.com/problems/takkfyrirmig

import Control.Monad

main = do
  a <- getLine
  let lineAmount = read a :: Int

  lines <- replicateM lineAmount getLine
  solution lines

solution list
  | null list = return ()
  | otherwise = do
      putStrLn ("Takk " ++ head list)
      solution (tail list)
