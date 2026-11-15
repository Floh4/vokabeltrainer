# Plan

## 2. rest

- [x] add new vocabulary

- [ ] check vocab:
  - [x] function to get words(everytime the function was succesfully called it should add 1 to the vocab check count)
  - [x] function that gives a random word and pass that word to the get word function
  - [ ] ~~update function(calculates the succes rate, etc.)~~
  - [ ] make an ui


### Feature ideas

- [ ] A Column 'Language' to specify the language (sql constraint NOT NULL)
- [ ] A Column 'Group' to specify eg. the chapter of vocab (can be null)
- [ ] A Column for extra information about the vocab(can be null)[^1]
- [ ] A Column for extra information about the translation(can be null)[^1]
- [ ] A Column for general information (can be null) [^1]

|Vocabulary|Translation|...|Language|Group|VocabInfo|TranslationInfo|Info|
|--|--|--|--|--|--|--|--|
|City|Stadt|...|English|Old||A Place with many Houses|eg. Munich|
|dicere|sagen, sprechen|...|Latin|New|dico, dixi, dictum|

[^1]: print the information when asking for the vocab

### Decisions
- [x] select database system 
  - ~~Network DB (MySQL etc.)?~~
  - SQLite simple local DB?
- [ ] User interface
  -  Terminal: [ratatui](https://github.com/ratatui/ratatui)
  -  GUI (e. g. [egui](https://github.com/emilk/egui), looks simple)

# Done

## 1. initialize function

- [x] make db at ~/.local/share/vocab_trainer/vocab.db

### db tables

- [x] for now: vocab, translation


#### Provide statistics (success rate)
|German|English|vocab check count|corect|
|--|--|--|--|
|Hund|Dog|10|9|
 |Nachbar|Neighbour|3|1|

Vocabulary with low success rate could be chosen more often for testing.

vocab check count is the amount of times the vocab got checked and corect is the count how often you got it corect.
You then can calculate the success rate.

#### Allow also not fully correct solutions

One could allow solutions with small typos as partially correct. For example, if the
given solution has just a typo like "neigbour", just add 0.8 to `correct` value. `correct` then 
should be a floating value.

For correctness something like [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) could be used. I guess there are libraries available for that.


## 3. technical design

### DB schema
First go for a naive approach? Simple table with one language
|German|English|
|--|--|
|Hund|Dog|

Mabe more columns with more languages?
