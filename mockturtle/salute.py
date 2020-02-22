#!/usr/bin/env python
# vi: set fileencoding=utf-8

import random
import sopel.bot as bot
import sopel

lobsters = [
    '( °,.,.,°)',
    '(V),oo,(V)',
    "(V) (',,,') (V)",
#    "(\/) (',,,,') (\/)",
#    '(\/) (°,.,°) (\/)',
#    'v.,,.v V.v.V v.,,.v',
#    'v.^.v',
#    '>.v.<',
#    "^`*w*'^",
#    '(\_\-/_/)',
    '(\/)_!_!_(\/)',
#    '()_\/_()',
#    '(\/).v.(\/)',
#    '\/_v_\/',
    'Λ˙^˙Λ',
]

claws = [ "(V)", "(\/)", "(\_/)", "V", "v", "(v)" ]
faces = [ "_!_!_", ".v.", "_00_" ]

@sopel.module.rule(r'^V\.v\.V$')
@sopel.module.commands('salute')
def salute(bot, trigger):
    """make a #lobsters salute"""
    lclaw, rclaw = random.sample(claws, 2)
    r = lclaw + random.choice(faces) + rclaw
    bot.say(r)

#@sopel.module.rule(ur'^Λ˙\^˙Λ$')
#def salute(bot, trigger):
#    """make a #lobsters salute"""
#    bot.say(ur'Λ˙^˙Λ')
