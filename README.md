## Genetic Rockets Simulation
A genetic algorithm simulation of rockets trying to reach the target written in [rust](https://www.rust-lang.org/) using [nannou](https://nannou.cc/).

#### Usage
- Clone the repo
    ```
    git clone git@github.com:sujay-ee/rust-genetic-rockets.git
    cd rust-genetic-rockets
    ```
- Run the simulation
    ``` 
    cargo run --release
    ```
- To update the simulation configurations use the configs file located at `src/configs.rs`
- To update the map layout edit the file at `assets/map.txt`. `0` indicates a wall, `1` indicates a no-wall block


#### References
- Python implementation of the same
  - https://github.com/sujay-ee/genetic-rockets-simulation
- Vectors
    - https://www.mathsisfun.com/algebra/vectors.html
    - https://natureofcode.com/book/chapter-1-vectors
- Autonomous Agents (Controlling rockets on the screen) -
    - https://natureofcode.com/book/chapter-6-autonomous-agents/
- Genetic Algorithm
    - https://natureofcode.com/book/chapter-9-the-evolution-of-code/
    - Youtube tutorial - https://www.youtube.com/watch?v=9zfeTw-uFCw&list=PLRqwX-V7Uu6bJM3VgzjNV5YxVxUwzALHV
