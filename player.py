
from collections import defaultdict
import hashlib
import os
import random
import re
import sys

import pexpect

def find_location(text):
    return hashlib.sha256(''.join(text.split('\r\n'))[:2].encode()).hexdigest()

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
    history = defaultdict(list)
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
            sys.exit(1)
        output = child.after.decode()
        location_id = find_location(output)
        items = find_list(output, 'Things of interest here:')
        options = find_list(output, 'There (is|are) \d+ exits?:')
        if not options:
            child.sendline('look')
            continue
        for i in items:
            print('Location: {} Take: {}'.format(location_id, i))
            child.sendline('take ' + i)
            continue
        unvisited = [o for o in options if o not in history[location_id]]
        if unvisited:
            options = unvisited
        try:
            choice = random.choice(options)
        except:
            print(output)
            print(options, unvisited)
            print('Exiting on exception in random.choice')
            sys.exit(1)
        else:
            history[location_id].append(choice)
            child.sendline(choice)
            print('{}: Location: {} Options: {} Unvisited: {} Chose: {}'.format(move, location_id, options, unvisited, choice))


if __name__ == '__main__':
    run()