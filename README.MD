# Rust Rubiks Cube Solver
# NOTE!! Doing a rebuild with a gui and more efficent algorithm so build is not working.
A simple rubiks cube solver that will hopefully be extended to a graphical interface. Used as a ~~simple~~ not-so-simple program to learn the basics of rust.

<!--![Screenshot of solve.](readme_resources/demo.png)-->

A rubiks cube solver that relies on a terminal interface. Solves any valid cube in "decent" time, by modelling the cube as a symmetric group. Performance can be improved by adding a pruning table, which I might one day do... along with a better UI. Ultimately it solves the any cube in a reasonable amount of moves which is good enough for me right now.

## Running
Run 
```sh
cargo run main
```
then follow the on screen instructions.
