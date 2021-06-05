# MinCost
A collection of modern heuristic optimization framework in Rust.

## Overwiew

1. Genetic Algorithm

2. Simulated Annealing(TODO)

3. Particle Swarm(TODO)

4. Tabu Search(TODO)


## Features

* Easy to Embed

  This is the primary design goal of mincost. See how to embed generic algorithm into your Rust project, please refer to [examples](examples/ga_examples)

* Various way to encoding your solution

  In mincost, you can encode your solution with various way. ie, in i8, i16, i32 and even boolean.

* Bounded solution

  In most combinational optimization problems, the solution is encoded as integer with lower-upper bounded. 
  mincost allows to configurate as bounded solution. Refer to [examples](examples/ga_examples)



## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)



