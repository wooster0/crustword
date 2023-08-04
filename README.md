# crustword - Crusty Crosswords

![image](https://user-images.githubusercontent.com/89065441/139863074-101ae732-9e63-4dcf-970d-29d0018707c6.png)

## Features

* [X] Solves crosswords
* [X] Generates random crosswords from a word list
* [X] Nice crossword output
* [X] Supports all kinds of languages
* [X] Spectator mode

## Usage

```
cargo install crustword
crustword
```

You can either generate a crossword or solve a crossword by passing a file that contains the crossword.

## Rules

In this kind of crossword, a grid of arbitrary size is filled with characters and the goal is to find all words in it from a specific list of words.

Consider this crossword:

```cr
Q C R O S S W O R D S
C R U S T Y E R T I O
P U A S D F G H J K L
Z S X C V E N M Q W E
R T T U I O L A S D F
G W H J K L Z Z C V B
N O M Q W E R T Z U I
O R P A S D F G H U K
L D Z X C V B N M Q P
```

* CRUSTWORD
* CRUSTY
* CROSSWORDS
* PUZZLE

Here is the solution with the relevant letters written in lowercase for distinction:

```cr
Q c r o s s w o r d s
c r u s t y E R T I O
P u A S D F G H J K L
Z s X C V e N M Q W E
R t T U I O l A S D F
G w H J K L Z z C V B
N o M Q W E R T z U I
O r P A S D F G H u K
L d Z X C V B N M Q p​
```
<!-- The above code block content ends with a zero-width space to make the last 'p' have the correct color -->

Words can be written in all eight directions: the four cardinal directions and the four ordinal directions.

You can find this crossword in `crosswords/`: [`crosswords/crustword/`](crosswords/crustword/)

## Input format

First comes a grid of characters with an arbitrary width and height.
Examples:

* ```
  aaa
  aaa
  aaa
  ```
* ```
  a a a
  a a a
  a a a
  ```
* ```
  a   a   a

  a   a   a

  a   a   a
  ```
etc.
The grid is very flexible and these are all the same.

On the last line comes a list of all words to be found in this grid separated by any amount of whitespace.
Examples:
* ```
  house tree shop
  ```
* ```
  house  tree  shop
  ```
* ```
  house   tree   shop
  ```
etc.
These are all the same.

Here is a full example:

```
AsAAt
AhAAr
house
ApAAe

house tree shop
```

Note that the matching algorithm is case-sensitive and "Word" does not match "word".

You can use any characters in your crustwords. Full-width as well as half-width characters are supported.

Original first commit on Fri Mar 18 18:41:39 2022
