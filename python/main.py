import numpy as np
import pygame, random

def draw(screen, arr, scale):
    pygame.draw.rect(screen, (0,0,0), pygame.Rect(0, 0, screen.get_width(), screen.get_height()))

    for y,i in enumerate(arr):
        for x,j in enumerate(i):
            if j > 0:
                color = pygame.Color(0)
                color.hsla = (j,100,50,100)
                pygame.draw.rect(screen, color, pygame.Rect(x*scale, y*scale, scale, scale))

def fall(arr):
    newArr = np.zeros((arr.shape[1], arr.shape[0]), int)
    dir = random.choice([1,-1])

    for y,i in enumerate(arr):
        for x,j in enumerate(i):
            if j > 0:
                if y+1 < arr.shape[0]:
                    if arr[y+1][x] == 0:
                        newArr[y+1][x] = arr[y][x]

                    elif x+1 < arr.shape[1] and dir == 1:
                        if arr[y+1][x+1] == 0:
                            newArr[y+1][x+1] = arr[y][x]
                        else:
                            newArr[y][x] = arr[y][x]
                        
                    elif x-1 >= 0 and dir == -1:
                        if arr[y+1][x-1] == 0:
                            newArr[y+1][x-1] = arr[y][x]
                        else:
                            newArr[y][x] = arr[y][x]
                        
                    else:
                        newArr[y][x] = arr[y][x]
                    
                else:
                    newArr[y][x] = arr[y][x]

    return newArr

winX = 400
winY = 400
scale = 5
FPS = 60

grid = np.zeros((winX//scale, winY//scale), int)
spawn = 0
color = 0
clicked = False

pygame.init()
window = pygame.display.set_mode((winX, winY))
clock = pygame.time.Clock()
run = True

while run:
    events = pygame.event.get()
    for event in events:
        if event.type == pygame.QUIT:
            run = False
        if event.type == pygame.MOUSEBUTTONDOWN:
            clicked = True
        if event.type == pygame.MOUSEBUTTONUP:
            clicked = False
    
    if spawn > 5:
        if color > 359:
            color = 0

        grid[0][int(winX/scale/10)] = color
        grid[0][int(winX/scale/2)] = color

        if clicked:
            mouseX, mouseY =  pygame.mouse.get_pos()
            grid[int(mouseY/scale)][int(mouseX/scale)] = color

        spawn = 0

    grid = fall(grid)
    draw(window, grid, scale)

    spawn += 1
    color += 1

    pygame.display.update()
    clock.tick(FPS)
pygame.quit()
