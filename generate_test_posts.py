import sys
import os
with open('posts/notes/eulerian_fluid_sim_p1.md') as f:
    data = [line.rstrip() for line in f.readlines()]

replace = 'path: "notes/eulerian_fluid_sim_p1_{}"'

if not os.path.exists('posts/temp'):
    os.mkdir('posts/temp')

for i in range(int(sys.argv[1])):
    with open('posts/temp/' + str(i) + '.md', 'w') as f:
        data[1] = replace.format(i)
        for line in data:
            print(line, file=f)
