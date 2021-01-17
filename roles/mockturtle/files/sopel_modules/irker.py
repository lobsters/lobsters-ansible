#!/usr/bin/env python
# vi: set fileencoding=utf-8
from sopel.config.types import StaticSection, ValidatedAttribute
from sopel.tools import events

import sopel.bot as bot
import sopel

from urllib.parse import urlparse

import atexit
import json
import logging
import socket
import sys

class IrkerSection(StaticSection):
    listen_port = ValidatedAttribute('listen_port', int)

def setup(bot):
    bot.config.define_section('irker', IrkerSection)

def close_socket(logger, socket):
    logger.info('Cleaning up irker socket.')
    socket.close()

@sopel.module.event(events.RPL_WELCOME)
@sopel.module.rule('.*')
@sopel.module.thread(True)
def irk(bot, trigger):
    logger = logging.getLogger('sopel')

    logger.info('Starting irker socket on port %d.', bot.config.irker.listen_port)

    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(("127.0.0.1", bot.config.irker.listen_port))

    atexit.register(close_socket, logger, sock)

    while True:
        data, addr = sock.recvfrom(1024)

        try:
            obj = json.loads(data)
            to = [obj['to']] if isinstance(obj['to'], str) else obj['to']

            for url in to:
                parsed_url = urlparse(url)
                channel = '#{}'.format(parsed_url.fragment if parsed_url.path == '/' else parsed_url.path)

                logger.debug('Sending `%s` to `%s`', obj['privmsg'], channel)
                bot.say(obj['privmsg'], channel)

        except ValueError:
            logger.info('Unable to decode input Received: %s.', data)
        except OSError as err:
            logger.error('OS Error encountered: %s.', err)
            sys.exit(409)
