#!/usr/bin/env python
# vi: set fileencoding=utf-8

import random
import sopel.bot as bot
import sopel

quotes = [
#    "I've performed a few mercy killings.",
#    'You seem malnourished. Are you suffering from internal parasites?',
#    "Ohhh, it's all so complicated, with the flowers, and the romance, and the lies upon lies.",
#    'This "love" intrigues me. Teach me to fake it.',
#    "it's been years since medical school, so remind me. Disemboweling in your species, fatal or non-fatal?"
#    "Of course I'm being irrational! I'm in love!",
#    "What? Now there's a bird involved?",
    "It's a fabulous, crabulous day!",
#    'What smells like methane?',
#    "Yippy ki yay. Guess where I've been.",
#    "We'll need to have a look inside you with this camera.",
#    "Friends! Help! A guinea pig tricked me!",
    "What? How do you spell that?",
    "It's clever, but that doesn't explain anything.",
    "If only it'd work, you could go back and not waste your time on it.",
#    "I wasn't wearing it. I was eating it.",
    "Let's face it, we're in hot butter here.",
    "A fancy dress gala? I'll wear my formal shell.",
    'Oh, right, right, with the bones. I always forget about the bones...',
    "I can't believe everybody's just ad-libbing!",
    "Hooray! I'm helping!",
    'See, this is how it starts. First with the jokes, then comes the heavy stuff.',
    'Tell me, is there any real soup, or just this schlock?',
    'What a day that was!',
    "And that's the story of how I got my new shell.",
    'Friends, look! I have barnacl.es in my tuckus.',
    'What about the hero of our story? Will I be invisible forever?',
    "I've had it with this game. I'm going for a scuttle.",
    'Looks like everyone gets a turn to say something. This concludes my turn.',
    'Hooray! People are paying attention to me.',
    "It's so cute. Horseshoe crab cute.",
]

@sopel.module.commands('zoidberg')
def zoidberg(bot, trigger):
    """Quote Zoidberg from Futurama"""
    bot.say(random.choice(quotes))
