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
| [19](https://adventofcode.com/2023/day/19) | Aplenty                         | 67.0µs | 110µs  | [day19.rs](./src/day19.rs) | :woozy_face:   |
| [20](https://adventofcode.com/2023/day/20) | Pulse Propagation               | 5.00ms | 21.0ms | [day20.rs](./src/day20.rs) | :sob:          |
| [21](https://adventofcode.com/2023/day/21) | Step Counter                    | 2.31ms | 120ms  | [day21.rs](./src/day21.rs) | :persevere:    |
| [22](https://adventofcode.com/2023/day/22) | Sand Slabs                      | 14.4µs | 10.3ms | [day22.rs](./src/day22.rs) | :confounded:   |
| [23](https://adventofcode.com/2023/day/23) | A Long Walk                     | 20.1µs | 2.351s | [day23.rs](./src/day23.rs) | :cursing_face: |
| [24](https://adventofcode.com/2023/day/24) | Never Tell Me The Odds          | 858µs  | 2.638s | [day24.rs](./src/day24.rs) | :roll_eyes:    |
| [25](https://adventofcode.com/2023/day/25) | Snowverload                     | 40.3ms | ------ | [day25.rs](./src/day25.rs) | :nerd_face:    |

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
15. Day 21 spent far too long trying to bug hunt, and it turns out it wasn't a math problem, it was a very basic problem involving the features I'm excluding.
16. Getting false positives in Day 22 tests was not helping matters much.
17. Day 23 saw some major procrastination where I ended up watching all of The Recruit and 3 Body Problem. The initial successful debug run took 254.7s! Unsure how to shoehorn in rayon for extra brute force. Must try the NRMCMC SAW algorithm at some point.
18. Day 24 was just awful, and more of an exercise in high level maths than an actual coding problem.
19. Day 25, as with any Monte-Carlo simulation, success depends on the number of runs. I have managed to get a successful run with just 10 iterations, but to improve odds, an increased number would be needed, but it's still no guarantee. 10 runs on single thread takes about 40ms, but for a larger number of runs I have parallelised the iterator to get through runs faster. Also, there's no part 2 for the last day.