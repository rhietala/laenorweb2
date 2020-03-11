module Lib
       ( surroundingsAt
       , coloredSurroundingsAt
       ) where

import Data.List

terrainColor :: Char -> String
terrainColor c
  | c `elem` ['!', '?', '%', 't'] = "white"
  | c `elem` ['#', '+', '-', '|', '/', '\\', '=', 'C', 'c'] = "light_black"
  | c `elem` ['y', 'z', 'b', 'd'] = "yellow"
  | c `elem` [','] = "light_yellow"
  | c `elem` ['.', 'F', 'v', 'j'] = "green"
  | c `elem` ['f'] = "light_green"
  | c `elem` ['V'] = "red"
  | c `elem` ['@', 'L', 'x', 's'] = "light_red"
  | c `elem` ['H', 'h'] = "magenta"
  | c `elem` ['^'] = "light_magenta"
  | c `elem` ['~'] = "blue"
  | c `elem` ['R', 'r', 'i', 'l'] = "light_blue"
  | c `elem` ['S', 'w'] = "cyan"
  | otherwise = ""

colorizeTerrain :: Char -> String
colorizeTerrain c =
  "<span class=\"" ++ terrainColor c ++ "\">" ++ [c] ++ "</span>"

terrainAt :: String -> Int -> Int -> Char
terrainAt d y x = (!!) ((!!) (lines d) (y - 1)) (x - 1)

surroundingsAt :: String -> Int -> Int -> Int -> Int -> [[Char]]
surroundingsAt d dx dy x y =
  map (\y2 -> map (terrainAt d y2) [(x-dx)..(x+dx)]) [(y-dy)..(y+dy)]

coloredSurroundingsAt :: String -> Int -> Int -> Int -> Int -> String
coloredSurroundingsAt d dx dy x y =
  intercalate "<br>"
  (map (\y2 -> intercalate ""
               (map (\x2 -> colorizeTerrain (terrainAt d y2 x2))
                [(x-dx)..(x+dx)]))
   [(y-dy)..(y+dy)])
