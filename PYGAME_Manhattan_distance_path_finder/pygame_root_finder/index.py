#Generating grid-space:
gamegrid = [[ 0 for x_c in range(10)  ] for y_c in range(10)]
#User trying to Escape (UTE) from danger zone (x,y):
ute = (0,0) #This is represented by ( 2 ) in the grid:
#Danger zone:
#Always Include the UTE's location in this array:
danger_zone = [[0,0],[0,1],[1,1],[1,0],[2,0],[0,2],[5,5]]
escape_nodes = []

#Filling gamegrid:
for y in range(len(gamegrid)):
    for x in range(len(gamegrid)):
        for d_z in danger_zone:
            if x == d_z[0] and y == d_z[1]:
                gamegrid[y][x] = 1
        if x == ute[0] and y == ute[1]:
            gamegrid[y][x] = 2

#Checking for possible spots outside of grid: ( 3 )
for y in range(len(gamegrid)):
    for x in range(len(gamegrid)):
        check_possib = [(-1,1) , (0,1) , (1,1) ,(-1,0) , (1,0) , (-1,-1) , (0,-1),(1,-1)]

        if gamegrid[y][x] == 0:
            #Occupancy is the local nodes arround X,Y to determine if this node is next to the dangerzone or not..
            occupancy = 0
            for check in range(8):

                try:
                    if gamegrid[y+check_possib[check][1]][x+check_possib[check][0]] == 1:
                        occupancy += 1
                except IndexError:
                    pass
            if occupancy > 0:
                escape_nodes.append([x,y])
                gamegrid[y][x] = 3

#What Node can I go to using manhattan distance to exit the dangerzone in the least amount of moves?
for n in escape_nodes:
    path_diff = ( abs(ute[0]-n[0]) ,abs(ute[1]-n[1]) )
    print(f"node:{n} , moves needed: {path_diff[0]+path_diff[1]}")


#Printing:
for p in gamegrid:
    print(p)



