# Specification

### Repo

[Link to repo](https://github.com/IndaPlus24/davidn5-mja8-projekt)

### Project Outline

The project is an attempt to recreate the popular game Tetris. Furthermore the project is deemed succesful if the the final product is able to played both on a normal computer as well as the "Drifarkaden" situated in the student facility "Meta".

### Requirments

#### Game mechanics

- 10 x 20 grid (standard Tetris board)
- Seven different shapes
- Pieces should fall from top and move downward automatically
- The player is able to move pieces from left, right, and down.
- Pieces should be able to rotate
- Line is cleared when row is fully occupied
- Scording system
- Increasing difficulty
- Game over when pieces are unable to spawn
- 1v1 Game Mode

#### Controls

- Controls for a normal keyboard (arrow keys or asd)
- Controls for Drifarkaden

#### UI

- Display pieces
- Show current sqcore and level
- Game start (restart option)
- Visual effect for cleared line
- High Score

#### Persistence

- Save high scores (top 10)
- Change game settings (controlls)

### Minimum Viable Product (MVP)

The MVP for the product is a function game of tetris containing most of the requirments listed above. (Its possible that the game is not visually appeling enough to be displayed in Drifarkaden).

### Work Process

The work will be distributed equally, and more complex issues will be tackled collaboratively.

### Work Distribution

Issue's and the implementation of features will be delegated as the project progresses.

### Workflow

#### Commit

> LABEL: <`description`>

Where LABEL is

- bug: report bug (in issue)
- doc: Addition to documentation
- feat: Implementation of new Feature
- fix: Fix of bug
- gui: Code related to gui
- func: Code related to functionality

#### Branching

All branches will be subject to a pull-request that has to be accepted by another team member. For branching use `git switch -c <name>` and push using `git push --set-upstream origin <branch>`

#### Issues

Issues will be identified and added throughout development. Refrencing said issues will be done in PR's when applicable.

#### Technologies

The project will be written in the language `Rust`. Furthermore it will use the GUI library `ggez` version `0.9.3`

#### Risks and Bottlenecks
The `requirements` above and the MVP shouldn't pose any larger problems and such we see no risks with those. The `Nice to have` listed below are quite a lot more difficult that the `requirements` and such could pose risks.


#### Nice to have
- Bots you can play against
- Good looking UI

