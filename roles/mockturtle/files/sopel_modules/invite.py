#!/usr/bin/env python
# vi: set fileencoding=utf-8

import datetime
import time

from sopel.module import event, rule, rate, unblockable

@rule(r'.*[Ii]nvite.*')
@rate(900)
def invite(bot, trigger):
    """To new members that are looking for an invitation"""
    timestamp = bot.db.get_nick_value(trigger.nick,
                                      'join_channel_' + trigger.sender)
    if not timestamp:
        return

    saw = datetime.datetime.utcfromtimestamp(timestamp)
    delta = (trigger.time - saw).total_seconds()

    if delta >= 3600:
        return

    bot.reply("If you would like an invite to lobste.rs, please look "
              "at the chat FAQ first. Thank you. https://lobste.rs/chat "
              "(I am a bot. (v).v.(v))")

@rule('(.*)')
@event('JOIN')
@unblockable
def note(bot, trigger):
    if not trigger.is_privmsg:
        bot.db.set_nick_value(trigger.nick,
                              'join_channel_' + trigger.sender,
                              time.time())

