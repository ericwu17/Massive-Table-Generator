# Massive Table Generator for optimal solutions to each state on the 2x2 Rubik's Cube

This program generates a massive table (`massive_table.bin`) of 3674160 entries,
each entry 6 bytes long, where each entry encodes the optimal solution to some state
on the 2x2 Rubik's Cube. Each solution is at most 11 moves long 
(11 is God's number for the 2x2 Rubik's Cube in the half-turn metric).

There are 3674160 possible states on the 2x2 cube if you fix one of the corners.
Fixing a corner is the same as considering two permutations as equivalent if they can be
reached by cube-rotations.
I chose to fix the DBL corner which means that solutions only involve moves of the 
F, U, and R faces.
The number of states is calculated by `7! * 3^6` since there are `7!` ways
to permute the 7 corners, and `3^6` possible orientations (the orientation of the last
corner is determined by the orientation of all other corners).

Each state can be represented as an array of 14 integers,
where the first 7 numbers describe the permutation state, and the last 7 numbers
describe the orientation state. Each state can also be encoded as an integer in the range
`0..(3674160-1)`. This integer is used as the index into `massive_table.bin` when
you need to look up the solution.

This program also generates a `massive_table.txt` file that contains all the entries
in a human-readable format, sorted by solution length. This `.txt` file
is not saved to the git repository since it is much larger than `massive_table.bin`
(187 MB compared to 22 MB).

The solutions were found by doing a simple BFS on the space of all possible 
states on the 2x2 cube.