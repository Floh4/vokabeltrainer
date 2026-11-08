# Plan

## 1. initialize function

- [x] make db at ~/.local/share/vocab_trainer/vocab.db

### db tables

- [x] for now: vocab, translation

## 2. rest

- [ ] add new vocabulary

- [ ] check vocab

### Feature ideas

#### Provide statistics (success rate)
|German|English|success rate|vocab check count|corect|
|--|--| --|--|--|
|Hund|Dog|0.9|10|9|
 |Nachbar|Neighbour|0.3|3|1|

Vocabulary with low success rate could be chosen more often for testing.

vocab check count is the amount of times the vocab got checked and corect is the count how often you got it corect.
You then can calculate the success rate.


## 3. technical design

### DB schema
First go for a naive approach? Simple table with one language
|German|English|
|--|--|
|Hund|Dog|

Mabe more columns with more languages?

### Decisions
- [x] select database system 
  - ~~Network DB (MySQL etc.)?~~
  - SQLite simple local DB?
- [ ] User interface
  -  Terminal: [ratatui](https://github.com/ratatui/ratatui)
  -  GUI (e. g. [egui](https://github.com/emilk/egui), looks simple)
