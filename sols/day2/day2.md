# AoC 2023- Day 2

## Problem Statement (Reworded)

The [problem statement](https://adventofcode.com/2023/day/2) for today is pretty clear after consulting the examples. The gist is that we need to do some calculations based off of the minimum number of different colored cubes needed for a series of draw-states to be possible.

## Part 1: 'What is the sum of all of the valid Game IDs'

We define a valid game ID to be one wherein the RGB values are less than or equal to `[12,13,14]` respectfully.

## Python

## Rust

## Part 2:'What is the product of the minimum number of cubes needed for each game?'

Here we calculate the minimum number of cubes of each color needed for a given game, and then multiply them together. Note that its possible to unintentionally zero the whole product out if just one of the terms zeros out- a strict reading of the requirements reveals that one should not do this, i.e. if you only need Red and Green cubes for a given Game, that is fine and this should be accounted for.

## Python

## Rust
