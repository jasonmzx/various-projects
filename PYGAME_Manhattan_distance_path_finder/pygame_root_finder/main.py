import pygame
pygame.init()
import math
import os

WIDTH, HEIGHT = 1000,1000

GRIDSIZE = 12

SQUARE_SIZE = math.ceil((WIDTH*0.75)/GRIDSIZE)


screen = pygame.display.set_mode((WIDTH,HEIGHT))
pygame.display.set_caption('Escape the Danger!')

FPS = 60

#UTE:
ute = [0,0]
#UTE Sprite:
SPRITE_IMG = pygame.image.load(os.path.join('assets','sprite.png'))
SPRITE = pygame.transform.scale(SPRITE_IMG,(SQUARE_SIZE,SQUARE_SIZE))

#TOGGLE ARRAY:
#0: Draw Dangerzone, 1: Move Sprite
TOGGLE = [1,0,0]


#Game Grid as an array:
gamegrid = [[ 0 for x_c in range(GRIDSIZE)  ] for y_c in range(GRIDSIZE)]
font = pygame.font.SysFont(None, 24)

def draw_screen(sq_check,node_checked,close_nodes):
    light_blue = [137,209,230]
    screen.fill(light_blue)
    #Draw Menu:

    #Dangerzone toggle:
    pygame.draw.rect(screen, (255,120,120),(125, 790, 150, 50),0)
    screen.blit((font.render('Draw Dangerzone', True, (0,0,0))), (130, 805))
    pygame.draw.rect(screen, (0,0,0),(125, 790, 150, 50),1)

    #Sprite toggle:
    pygame.draw.rect(screen, (120,255,120),(125, 850, 150, 50),0)
    pygame.draw.rect(screen, (0,0,0),(125, 850, 150, 50),1)
    screen.blit((font.render('Move Sprite', True, (0,0,0))), (150, 865))
    #Start Searching:
    pygame.draw.rect(screen, (164, 101, 160),(125, 910, 150, 50),0)
    pygame.draw.rect(screen, (0,0,0),(125, 910, 150, 50),1)
    screen.blit((font.render('Start Search', True, (0,0,0))), (150, 925))

    #Draw Grid:
    for gd_y in range(GRIDSIZE):
        for gd_x in range(GRIDSIZE):
            #If Cell is Empty:
            if gamegrid[gd_y][gd_x] == 0:
                pygame.draw.rect(screen, (255,255,255), (125+SQUARE_SIZE*gd_x, 20+SQUARE_SIZE*gd_y, SQUARE_SIZE, SQUARE_SIZE),0)
            #If Cell is Dangerzone
            if gamegrid[gd_y][gd_x] == 1:
                pygame.draw.rect(screen, (255,120,120), (125+SQUARE_SIZE*gd_x, 20+SQUARE_SIZE*gd_y, SQUARE_SIZE, SQUARE_SIZE),0)


            if gamegrid[gd_y][gd_x] == 3:
                pygame.draw.rect(screen,(186, 222, 93), (125+SQUARE_SIZE*gd_x, 20+SQUARE_SIZE*gd_y, SQUARE_SIZE, SQUARE_SIZE),0)

            #Draw Border:
            pygame.draw.rect(screen, (0, 0, 0),(125 + SQUARE_SIZE * gd_x, 20 + SQUARE_SIZE * gd_y, SQUARE_SIZE, SQUARE_SIZE), 1)

            if gd_x == ute[0] and gd_y == ute[1]:
                screen.blit(SPRITE, (125+gd_x*SQUARE_SIZE,20+gd_y*SQUARE_SIZE))

    if node_checked[0] == -1:
        pass
    else:
        pygame.draw.rect(screen, (0, 255, 0),(125 + SQUARE_SIZE * node_checked[0], 20 + SQUARE_SIZE * node_checked[1], SQUARE_SIZE, SQUARE_SIZE), 0)

    if sq_check[0] == -1:
        pass
    else:
        pygame.draw.rect(screen, (55, 255, 120),(125 + SQUARE_SIZE * sq_check[0], 20 + SQUARE_SIZE * sq_check[1], SQUARE_SIZE, SQUARE_SIZE), 0)

    if not close_nodes:
        pass
    else:
        for node in close_nodes:
            print(node)
            pygame.draw.circle(screen, (199, 199, 199), (125 + SQUARE_SIZE*node[0]+SQUARE_SIZE/2, 20 + SQUARE_SIZE*node[1]+SQUARE_SIZE/2), SQUARE_SIZE/3, 0)
            pygame.draw.circle(screen, (255, 0, 0), (125 + SQUARE_SIZE*node[0]+SQUARE_SIZE/2, 20 + SQUARE_SIZE*node[1]+SQUARE_SIZE/2), SQUARE_SIZE/3, 3)
    pygame.display.update()

escape_nodes = []
e_n_distance = []
close_nodes = []
def main():
    clock = pygame.time.Clock()
    run = True
    while run:
        clock.tick(FPS)
        sq_check = [-1,-1]
        node_checked = [-1,-1]
        checking = False
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                run = False
            if event.type == pygame.MOUSEBUTTONUP:
                #Draw Dangerzone Button:
                if 125 < pygame.mouse.get_pos()[0] < 275 and 790 < pygame.mouse.get_pos()[1] < 840 and checking is False:
                    TOGGLE[0] = 1
                    TOGGLE[1] = 0
                    TOGGLE[2] = 0
                #Sprite Mouver Button:
                if 125 < pygame.mouse.get_pos()[0] < 275 and 850 < pygame.mouse.get_pos()[1] < 900 and checking is False:
                    TOGGLE[0] = 0
                    TOGGLE[1] = 1
                    TOGGLE[2] = 0
                if 125 < pygame.mouse.get_pos()[0] < 275 and 910 < pygame.mouse.get_pos()[1] < 960 and checking is False:
                    TOGGLE[0] = 0
                    TOGGLE[1] = 0
                    if gamegrid[ute[1]][ute[0]] == 1:
                        print("Valid, proceeding to checker")
                        checking = True
                        #Escape check Insert the escape check animation/code here
                        for y in range(len(gamegrid)):
                            for x in range(len(gamegrid)):
                                check_possib = [(-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0)]
                                if gamegrid[y][x] == 0:
                                    # Occupancy is the local nodes arround X,Y to determine if this node is next to the dangerzone or not..
                                    occupancy = 0
                                    for check in range(8):
                                        node_checked = [x, y]
                                        try:
                                            if gamegrid[y + check_possib[check][1]][x + check_possib[check][0]] == 1:
                                                occupancy += 1
                                            sq_check = [x + check_possib[check][0], y + check_possib[check][1]]
                                        except IndexError:
                                            pass
                                        pygame.time.wait(1)
                                        draw_screen(sq_check, node_checked,close_nodes)
                                    if occupancy > 0:
                                        escape_nodes.append([x, y])
                                        gamegrid[y][x] = 3
                        print("checking the shortest paths:")
                        for n in escape_nodes:
                            path_diff = (abs(ute[0] - n[0]), abs(ute[1] - n[1]))
                            e_n_distance.append(path_diff[0]+path_diff[1])
                        min_dist = -1
                        #Find smallest Distance:
                        for z in e_n_distance:
                            if min_dist == -1:
                                min_dist = z
                            else:
                                if min_dist > z:
                                    min_dist = z
                        for n in range(len(escape_nodes)):
                            if e_n_distance[n] == min_dist:
                                close_nodes.append(escape_nodes[n])
                        print(f"Min DIst: {min_dist}")


                        checking = False


                if TOGGLE[0] == 1:
                    #print(f"clicked at {pygame.mouse.get_pos()[0]} ")
                    if 125 < pygame.mouse.get_pos()[0] < 875 and 20 < pygame.mouse.get_pos()[1] < 770 :
                        x_c = math.floor((pygame.mouse.get_pos()[0] - 125)/SQUARE_SIZE)
                        y_c = math.floor((pygame.mouse.get_pos()[1] - 20)/SQUARE_SIZE)
                        if gamegrid[y_c][x_c] == 0 or gamegrid[y_c][x_c] == 3:
                            gamegrid[y_c][x_c] = 1
                        elif gamegrid[y_c][x_c] == 1:
                            gamegrid[y_c][x_c] = 0
                        print(f"[CLICK!] User has click at X: {x_c},Y: {y_c}")
                    else:
                        print(f"[CLICK!] Offgrid, at {pygame.mouse.get_pos()[0]},{pygame.mouse.get_pos()[1]}")
                if TOGGLE[1] == 1:
                    if 125 < pygame.mouse.get_pos()[0] < 875 and 20 < pygame.mouse.get_pos()[1] < 770 :
                        x_c = math.floor((pygame.mouse.get_pos()[0] - 125)/SQUARE_SIZE)
                        y_c = math.floor((pygame.mouse.get_pos()[1] - 20)/SQUARE_SIZE)
                        ute[0] = x_c
                        ute[1] = y_c


        draw_screen(sq_check,node_checked,close_nodes)
        pass

    pygame.quit()

if __name__ == '__main__':
    main()


