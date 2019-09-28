#!/usr/bin/env python
# vi: set fileencoding=utf-8

import random
import sopel.bot as bot
import sopel

quadrille = [
    '"Will you walk a little faster?" said a whiting to a snail,',
    '"There\'s a porpoise close behind us, and he\'s treading on my tail.',
    'See how eagerly the lobsters and the turtles all advance!',
    'They are waiting on the shingle – will you come and join the dance?',
    "Will you, won't you, will you, won't you, will you join the dance?",
#    "Will you, won't you, will you, won't you, won't you join the dance?",
    'You can really have no notion how delightful it will be',
    'When they take us up and throw us, with the lobsters, out to sea!"',
    'But the snail replied "Too far, too far!" and gave a look askance --',
    'Said he thanked the whiting kindly, but he would not join the dance.',
    'Would not, could not, would not, could not, would not join the dance.',
#    'Would not, could not, would not, could not, could not join the dance.',
    '"What matters it how far we go?" his scaly friend replied.',
    'There is another shore, you know, upon the other side.',
    'The further off from England the nearer is to France --',
    'Then turn not pale, beloved snail, but come and join the dance.',
#    "Will you, won't you, will you, won't you, will you join the dance?",
#    "Will you, won't you, will you, won't you, won't you join the dance?",
]

@sopel.module.commands('quadrille', 'sing')
def sing(bot, trigger):
    """The Mock Turtle speaks a line from the Lobster Quadrille."""
    bot.say(random.choice(quadrille))
