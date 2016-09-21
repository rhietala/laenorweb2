{-# LANGUAGE OverloadedStrings #-}
module Main where

import Lib
import LocParser
import Text.Regex

test :: IO ()
test = do
  contents <- readFile "../data/lucentium.map"
  print  (Lib.coloredSurroundingsAt contents 2 1 413 153)

main :: IO ()
main = test
