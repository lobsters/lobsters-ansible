#!/usr/bin/env python
# vi: set fileencoding=utf-8

import sopel.bot as bot
import sopel

@sopel.module.rule(r'.*[Pp]eterson.*')
@sopel.module.rate(channel=300)
def peterson(bot, trigger):
    """Link to FAQ if Peterson is mentioned"""
    bot.reply("The site has nothing to do with Jordan Peterson: https://lobste.rs/about#michaelbolton")
