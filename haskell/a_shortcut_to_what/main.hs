-- A Shortcut to What?
-- https://open.kattis.com/problems/shortcuttowhat

main = do
  line <- getLine
  let num = read line :: Int
  print (((num + 5) * 3) - 10)
