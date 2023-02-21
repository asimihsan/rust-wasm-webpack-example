#!/usr/bin/env python3

#  Copyright (C) 2023 Asim Ihsan
#  SPDX-License-Identifier: AGPL-3.0-only
#
#  This program is free software: you can redistribute it and/or modify it under
#  the terms of the GNU Affero General Public License as published by the Free
#  Software Foundation, version 3.
#
#  This program is distributed in the hope that it will be useful, but WITHOUT ANY
#  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
#  PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
#
#  You should have received a copy of the GNU Affero General Public License along
#  with this program. If not, see <https://www.gnu.org/licenses/>

from pathlib import Path
from string import Template
from typing import List
import multiprocessing
import os.path
import subprocess

SCRIPT_DIR: str = os.path.dirname(os.path.realpath(__file__))
DIST_DIR: str = os.path.join(SCRIPT_DIR, "dist")
COMMAND = Template("critical $source --base $base --inline | sponge $source")


def run_command(command: str):
    subprocess.check_call(command, shell=True)


def main():
    commands: List[str] = []
    for path in Path(DIST_DIR).rglob("*.html"):
        source = path.absolute()
        command: str = COMMAND.substitute(source=source, base=DIST_DIR)
        print("processing %s..." % (source,))
        commands.append(command)
    with multiprocessing.Pool() as pool:
        pool.map(run_command, commands)


if __name__ == "__main__":
    main()
