# Ideas #
1. Use interpolated rotation
2. Allow sparse maps
3. Add GJK

# Continuous 2D Collision Detection #
0. Prune collisions by using sweep-and-prune
1. Use boolean GJK (bgjk)
2. Take two shapes defined by vertices
3. Add movement to vertices get new position
4. For each such pair (old_x, old_y) and (new_x, new_y) create (old_x, old_y, 0) and (new_x, new_y, 1).
5. Run bgjk on the shapes
6. If true we have collision
7. Obstacles: do not move if there is collision. Else move
8. Items: interact

# Continuous 3D Collision Detection #
0. Prune collisions by using sweep-and-prune
1. Use 4D boolean GJK (bgjk4d) to determine intersections using continuous motion

# Collision Resolution #
This has to be worked out
