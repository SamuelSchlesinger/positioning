# Positioning

A library for manipulating coordinates on a 3D grid. This will contain code
that I'll often end up repeating in games.

Why would you use this library? Well, the reason I worked on this code was
because I was running into major performance issues with pathfinding. First
I implemented Djikstra better, then I implemented A*, then I found this wonderful
heuristic for A* which is on display in the `pathfinding` module.

Lets say we have a static map of all of the unblocked tiles our characters are
able to move to. If characters are able to move through one another, then we
should use all-pairs shortest paths in order to get the quickest path very quickly.
On the other hand, if the players block each other, or other blockers may be
introduced dynamically, then we can't quite do that. When I had these thoughts,
I disappointedly implemented A* using the Hamming distance heuristic. While simmering
on my impotence, I realized that I could use all-pairs shortest paths as the heuristic,
as it is admissible. So I did that! And here we are.

Enjoy!
