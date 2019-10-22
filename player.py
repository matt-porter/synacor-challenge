
from collections import defaultdict
import hashlib
import os
import random
import re
import sys

import pexpect

def route_to_dot(route):
    uniq = set()
    with open('route.dot', 'w') as f:
        f.write('digraph g {\n')
        for ((choice1, loc_id1), (choice2, loc_id2)) in zip(route, route[1:]):
            if (loc_id1, loc_id2, choice2) not in uniq:
                f.write('{} -> {} [label="{}"];\n'.format(loc_id1, loc_id2, choice2))
                uniq.add((loc_id1, loc_id2, choice2))
        f.write('}\n')
#    pexpect.spawn('/usr/local/bin/dot -x -Tpng route.dot -o route.png')

def find_location(text):
    return 'L' + hashlib.sha256(''.join(text.split('\r\n'))[:2].encode()).hexdigest()

def find_list(text, heading):
    """Find the list items under heading in text"""
    match = re.search(heading, text)
    if not match:
        return []
    start = match.span()[1]
    lines = text[start:].split('\r\n')
    if not lines[0].strip():
        lines = lines[1:]
    options = []
    for line in lines:
        if not line.strip():
            break
        if line.startswith('-'):
            options.append(line[2:].strip())
    return options

def run():
    this_route = []
    child = pexpect.spawn('target/release/synacor')
    last_options = []
    for move in range(1000):
        try:
            child.expect(
                """.*What do you do\?"""
            )
        except pexpect.EOF:
            print(child.before.decode())
            print('exiting on EOF')
            this_route[-1] = ((this_route[-1][0], 'EOF'))
            print(route_to_dot(this_route))
            sys.exit(1)
        output = child.after.decode()
        location_id = find_location(output)
        # replace last element to incl. location id
        if this_route:
            this_route[-1] = ((this_route[-1][0], location_id))
        else:
            this_route.append(('START', location_id))
        items = find_list(output, 'Things of interest here:')
        options = find_list(output, 'There (is|are) \d+ exits?:')
        if not options:
            child.sendline('look')
            continue
        for i in items:
            print('Location: {} Take: {}'.format(location_id, i))
            child.sendline('take ' + i)
            this_route.append(('take ' + i, location_id))
            continue
        choice = random.choice(options)
        this_route.append((choice,None))
        child.sendline(choice)
        print('{}: Location: {} Options: {} Chose: {}'.format(move, location_id, options, choice))



if __name__ == '__main__':
    run()
