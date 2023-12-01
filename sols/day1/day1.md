# AoC 2023- Day 1

## Problem Statement (Reworded)

You have a "calibration document" which consists of lines of text. Each line _originally_ contained a specific calibration value that the Elves now need to recover. On each line it can be found by combining the first digit and last digit (in order) to form a single two digit number.

## Part 1: 'What is the sum of all of the calibration values'

The solution is relatively straightforward: for each line, iterate through, updating a "first" and "last" variable as you go. Add that to a "sum" variable.

## Python

## Rust

## Part 2:'Oops! Some of the digits can be spelled with letters!'

The only change here is that we need to adjust how we're parsing each line. Note that we can't just do a replace on the line for each digit: there is overlap between the start and end of some of them. This also makes doing it in one iteration a bit more annoying, so we'll use some nested loops.

## Python

## Rust
