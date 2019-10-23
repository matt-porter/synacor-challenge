
from collections import defaultdict
import hashlib
import os
import random
import re
import sys

import pexpect

def route_to_dot(route, path='route.dot'):
    uniq = set()
    with open(path, 'w') as f:
        f.write('digraph g {\n')
        for ((choice1, loc_id1), (choice2, loc_id2)) in zip(route, route[1:]):
            if (loc_id1, loc_id2, choice2) not in uniq:
                f.write('"{}" -> "{}" [label="{}"];\n'.format(loc_id1, loc_id2, choice2))
                uniq.add((loc_id1, loc_id2, choice2))
        f.write('}\n')
#    pexpect.spawn('/usr/local/bin/dot -x -Tpng route.dot -o route.png')

def find_location(title, text, options):
    h = 'L' + hashlib.sha256(''.join((text.split('\r\n'))[:2] + options).encode()).hexdigest()
    loc = title.strip('= ') + ' ' + h[:8]
    loc = re.sub(r'\!', '', loc, re.I)
    return loc

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

fixed_options = {
    "Dark cave L6a9ae70": 'north',
    "Foothills L8a259bf": 'doorway',
    "Rope bridge L9801bfb": 'continue',
}

def run():
    previous_choices = set()
    all_paths = []
    title = ''
    for attempt in range(50):
        child = pexpect.spawn('target/release/synacor')
        this_route = []
        for move in range(200):
            # if move == 990:
            #     child.interact()
            #     return
            try:
                i = child.expect(['==.*==', ".*What do you do\?"])
                if i == 0:
                    title = child.after.decode()
                    continue
            except pexpect.EOF:
                node = child.before.decode().split('\r\n')[-2]
                print('exiting on EOF')
                this_route[-1] = ((this_route[-1][0], node))
                route_to_dot(this_route, 'routes/route{:02}.dot'.format(attempt))
                break
            except pexpect.Timeout:
                node = child.before.decode().split('\r\n')[-2]
                print('exiting on Timeout')
                this_route[-1] = ((this_route[-1][0], node))
                route_to_dot(this_route, 'routes/route{:02}.dot'.format(attempt))
                break
            output = child.after.decode()
            items = find_list(output, 'Things of interest here:')
            options = find_list(output, 'There (is|are) \d+ exits?:')
            location_id = find_location(title, output, options)
            # replace last element to incl. location id
            if this_route:
                this_route[-1] = ((this_route[-1][0], location_id))
            else:
                this_route.append(('START', location_id))
            if not options:
                child.sendline('look')
                continue
            for i in items:
                print('Location: "{}" Take: {}'.format(location_id, i))
                child.sendline('take ' + i)
                this_route.append(('take ' + i, location_id))
                child.sendline('use ' + i)
                this_route.append(('use ' + i, location_id))
                if i == 'can':
                    child.sendline('use lantern')
                    this_route.append(('use lantern', location_id))
                continue
            # If there's a choice to take that we haven't taken before, try that.
            pre_determined = fixed_options.get(location_id)
            preferred = [choice for choice in options if (location_id, choice) not in previous_choices]
            if pre_determined:
                print('Pre')
                choice = pre_determined
            elif preferred:
                choice = random.choice(preferred)
            else:
                choice = random.choice(options)
            this_route.append((choice,None))
            previous_choices.add((location_id, choice))
            child.sendline(choice)
            print('{}: Location: "{}" Options: {} Chose: {}'.format(move, location_id, options, choice))
        all_paths.extend(this_route)
    route_to_dot(all_paths, 'all_paths.dot')


if __name__ == '__main__':
    run()
