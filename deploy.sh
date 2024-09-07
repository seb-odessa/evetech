#!/bin/bash

ZKBINFO=vps
BINARIES=$(find deploy/release -maxdepth 1 -type f -executable)
SCRIPTS="scripts start.sh stop.sh"

rsync -urP $SCRIPTS vps:~/zkbinfo/
rsync -uP $BINARIES vps:~/zkbinfo/bin/
