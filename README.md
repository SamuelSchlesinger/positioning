# Positioning

[**rustdocs**](https://samuelschlesinger.github.io/positioning/positioning/)

A library for manipulating coordinates on a 3D grid. This will contain code
that I'll often end up repeating in games. Currently, it is not on
[crates.io](https://crates.io), because I doubt anyone will have use for it but
me, but I am open to changing that if others use it. I've hosted the rustdocs
above so its no less convenient than if it were on
[crates.io](https://crates.io).

The main feature of the library is the `Position` type which I was reusing
across multiple projects. It's no more than a triple of `i64`s with some
convenient functionality around it:

```rust
let p: Position = (1, 2, 3i64).into();

for neighbor in p.adjacent() {
  assert!(p.is_adjacent_to(neighbor));
  assert!(p.hamming_distance(neighbor) == 1);
}
```

The above example showcases the key assumption of this library, and how you'll
know if its useful for your settings: it defines adjacency of two positions as
being equivalent to having Hamming distance 1 between them. Pathfinding, thus,
will find paths which take movements of Hamming distance 1 away, excluding
diagonal paths.

## Breadth First Search

We implement an iterator `Bfs<'a>` which, based on a set of passable positions,
return each position in distance order, disambiguating based on the order
defined on `Position`. The iterator also returns the distance at which the
reachable position was found.

This is helpful if you're trying to write code which searches for something,
such as a viable task to do within a certain distance, or an enemy which you can
begin to target. On the other hand, its very much encouraged that you pick a
target and use the `pathfinding` module after that, as hacking the internals
of this to provide paths would lose a lot of the benefits of this library, which
are described in the following section.


## Pathfinding

The reason I worked on this code was because I was running into major
performance issues with pathfinding. First I implemented Djikstra better, then
I implemented A\*, then I found this wonderful heuristic for A\* which is on
display in the `pathfinding` module.

Lets say we have a static map of all of the unblocked tiles our characters are
able to move to. If characters are able to move through one another, then we
should use all-pairs shortest paths in order to get the quickest path very
quickly.  On the other hand, if the players block each other, or other blockers
may be introduced dynamically, then we can't quite do that. When I had these
thoughts, I disappointedly implemented A\* using the Hamming distance
heuristic. While simmering on my impotence, I realized that I could use
all-pairs shortest paths as the heuristic, as it is admissible.

This heuristic is very helpful in many specific circumstances. Particularly, if
we have a static grid like the following:

```
______________________
|e X             start|
|n X                  |
|d X                  |
|  X                  |
|  X                  |
|  X                  |
|  X                  |
|  X                  |
|  X                  |
|  X                  |
|  X                  |
|  XXXXXXXXXXXXXXXXX  |
|                     |
-----------------------
```

Using A\* with the Hamming distance metric, we don't really gain too much in
this case over an implementation of Djikstra's algorithm. In particular, in
both cases, we might search the entire top right, enclosed quadrant before we
end up seeking a path below it. However, using the all pairs shortest paths
metric, we're able to save a lot. In fact, this is a cheat, as I haven't
specified any dynamic blockers, but it displays what the heuristic is doing for
us. In fact, there are many places where we could introduce dynamic blockers
which wouldn't even hurt our performance at all.

```
______________________
|e X             start|
|n X                  |
|d X        @         |
|  X                  |
|  X                  |
|  X     @            |
|  X                  |
|  X                  |
|  X  @               |
|  X                  |
|  X                  |
|  XXXXXXXXXXXXXXXXX  |
|                     |
-----------------------
```

Even if we introduce one directly in the way of the shortest path, we'll still
be searching through much less of the grid. In some common in-game cases, this
ends up taking the cost down (roughly) from quadratic to linear.

## Contributions and Forks

Contributions and forks are very welcome! Games have very different needs, and
I'm happy to add things here which make sense for my use cases and I'd also be
happy to review any forks.
