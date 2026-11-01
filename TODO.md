# Plan

## 1. initialize function

- [ ] make db at ~/.local/share/vocab_trainer/vocab.db

### db tables

- [ ] for now: vocab, translation

## 2. rest

- [ ] add new vocabulary

- [ ] check vocab

### Feature ideas

#### Provide statistics (success rate)
|German|English|success rate|
|--|--| --|
|Hund|Dog|0.9|
 |Nachbar|Neighbour|0.3|

Vocabulary with low success rate could be chosen more often for testing.


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
  -  Terminal
  -  GUI (e. g. [egui](https://github.com/emilk/egui), looks simple)


## Done

