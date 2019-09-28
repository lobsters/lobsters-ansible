#!/usr/bin/env python
# vi: set fileencoding=utf-8

import random
import sopel.bot as bot
import sopel

sob = [
    '::cries::',
    '::sobs::',
    '::sighs::',
]

# @sopel.module.rule(r'.*systemd.*')
# def systemd(bot, trigger):
#     """sob if systemd is mentioned"""
#     bot.say(random.choice(sob))
#
@sopel.module.rule(r'.*meltdown.*')
@sopel.module.rule(r'.*spectre.*')
def spectre(bot, trigger):
    """sob if spectre or meltdown are mentioned"""
    bot.say(random.choice(sob))
