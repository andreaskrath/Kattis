-- Keys, Phone, Wallet
-- https://open.kattis.com/problems/keysphonewallet

import Control.Monad

main = do
  sAmount <- getLine
  let amount = read sAmount :: Int
  list <- replicateM amount getLine
  solution list False False False

solution [] keys phone wallet = do
  unless keys (putStrLn "keys")
  unless phone (putStrLn "phone")
  unless wallet (putStrLn "wallet")
  when (keys && phone && wallet) (putStrLn "ready")
solution list keys phone wallet =
  let current = head list
   in if current == "keys"
        then solution newList True phone wallet
        else
          if current == "phone"
            then solution newList keys True wallet
            else
              if current == "wallet"
                then solution newList keys phone True
                else solution newList keys phone wallet
  where
    newList = tail list
