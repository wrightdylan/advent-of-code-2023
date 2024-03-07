# :gift::christmas_tree: Advent of Code 2023 :christmas_tree::sparkles:

These are my solutions to this year's [Advent of Code](https://adventofcode.com/2023/).

Solutions make use of `cargo-aoc` code helper ([here](https://github.com/gobanos/cargo-aoc)).

## Solutions

All solutions linked below:
| Day | Title | 1 :star: | 2 :star: | Solution | Rating |
|:-|:-|:-|:-|:-|:-|
| [01](https://adventofcode.com/2023/day/1)  | Trebuchet?!                     | 131µs  | 4.25ms | [day01.rs](./src/day01.rs) | :monocle_face: |
| [02](https://adventofcode.com/2023/day/2)  | Cube Conundrum                  | 1.75µs | 1.71µs | [day02.rs](./src/day02.rs) | :star_struck:  |
| [03](https://adventofcode.com/2023/day/3)  | Gear Ratios                     | 669µs  | 387µs  | [day03.rs](./src/day03.rs) | :weary::joy:   |
| [04](https://adventofcode.com/2023/day/4)  | Scratchcards                    | 2.45µs | 3.43µs | [day04.rs](./src/day04.rs) | :laughing:     |
| [05](https://adventofcode.com/2023/day/5)  | If You Give A Seed A Fertilizer | 9.57µs | 104s   | [day05.rs](./src/day05.rs) | :persevere:    |
| [06](https://adventofcode.com/2023/day/6)  | Wait For It                     | 2.71µs | 2.49µs | [day06.rs](./src/day06.rs) | :relaxed:      |
| [07](https://adventofcode.com/2023/day/7)  | Camel Cards                     | 499ms  | 472ms  | [day07.rs](./src/day07.rs) | :confounded:   |
| [08](https://adventofcode.com/2023/day/8)  | Haunted Wasteland               | 1.33ms | 8.67ms | [day08.rs](./src/day08.rs) | :scream:       |
| [09](https://adventofcode.com/2023/day/9)  | Mirage Maintenance              | 150ms  | 154ms  | [day09.rs](./src/day09.rs) | :thinking:     |
| [10](https://adventofcode.com/2023/day/10) | Pipe Maze                       | 462µs  | 698µs  | [day10.rs](./src/day10.rs) | :confounded:   |
| [11](https://adventofcode.com/2023/day/11) | Cosmic Expansion                | 1.63ms | 1.63ms | [day11.rs](./src/day11.rs) | :nerd_face:    |
| [12](https://adventofcode.com/2023/day/12) | Hot Springs                     | 1.60ms | 11.2ms | [day12.rs](./src/day12.rs) | :hot_face:     |
| [13](https://adventofcode.com/2023/day/13) | Point of Incidence              | 8.17µs | 809µs  | [day13.rs](./src/day13.rs) | :woozy_face:   |
| [14](https://adventofcode.com/2023/day/14) | Parabolic Reflector Dish        | 958µs  | 1.84s  | [day14.rs](./src/day14.rs) | :grimacing:    |
| [15](https://adventofcode.com/2023/day/15) | Lens Library                    | 45.4µs | 235µs  | [day15.rs](./src/day15.rs) | :blush::grin:  |
| [16](https://adventofcode.com/2023/day/16) | The Floor Will Be Lava          | 3.99ms | 286ms  | [day16.rs](./src/day16.rs) | :relieved:     |
| [17](https://adventofcode.com/2023/day/17) | Clumsy Crucible                 | 217ms  | 690ms  | [day17.rs](./src/day17.rs) | :woozy_face:   |
| [18](https://adventofcode.com/2023/day/18) | Lavaduct Lagoon                 | 12.6µs | 6.21µs | [day18.rs](./src/day18.rs) | :innocent:     |
<!--| [19](https://adventofcode.com/2023/day/19) | Aplenty                         |  |  | [day19.rs](./src/day19.rs) |  |-->
<!--| [20](https://adventofcode.com/2023/day/20) | Pulse Propagation               |  |  | [day20.rs](./src/day20.rs) |  |-->
<!--| [21](https://adventofcode.com/2023/day/21) | Step Counter                    |  |  | [day21.rs](./src/day21.rs) |  |-->
<!--| [22](https://adventofcode.com/2023/day/22) | Sand Slabs                      |  |  | [day22.rs](./src/day22.rs) |  |-->
<!--| [23](https://adventofcode.com/2023/day/23) | A Long Walk                     |  |  | [day23.rs](./src/day23.rs) |  |-->
<!--| [24](https://adventofcode.com/2023/day/24) | Never Tell Me The Odds          |  |  | [day24.rs](./src/day24.rs) |  |-->
<!--| [25](https://adventofcode.com/2023/day/25) | Snowverload                     |  |  | [day25.rs](./src/day25.rs) |  |-->

## Notes
1. Binge watching Travellers wasn't helping me keep up with the Advent Calendar. If only it was chocolate :chocolate_bar:.
2. Day 3 part 1 was a bit of a struggle, but part 2 took literally seconds :laughing:.
3. Day 5 part 2 was 273s single threaded, but was 104s multithreaded. I was hoping for more performance.
4. It turns out part tests work better when calling the correct function. Who knew?
5. It turns out functions work as intended when all branches are actually different. :man_facepalming:
6. That's enough of a hiatus between days 9 and 10.
7. I've got something similar to day 11 involving [star clusters](https://github.com/wrightdylan/cncalc).
8. Day 12 required learning more about non-deterministic finite automata and powerset construction. There is an excellent series by Neso Academy on YouTube called Theory of Computation & Automata Theory.
9. It took 3 iterations to get day 13 working right.
10. For Day 14 I used Floyd's and Brent's algorithms for cycle detection; the latter is 0.6s faster.
11. Day 15 was suspiciously easy. That's never a good sign. :thinking:
12. For day 16, this was my first time writing a depth-first search, and it ran perfectly on the first attempt :exploding_head:. Because I tend to write these solutions generalised for robustness, it effectively meant part 2 was already solved as well.
13. For day 17, I was originally going for Dijkstra's, but decided to needlessly overcomplicate things by usinga heuristic, and included a visualiser to see the shortest path discovered. This could be a bit faster though.
14. Day 18 basically reused the shoelace formula and Pick's theorem from day 10. It's not clear why part 2 consistnently run twice as fast as part 1 even though they both use the exact same function.