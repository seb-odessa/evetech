#!/bin/bash

ZKBINFO=vps
BINARIES=$(find deploy/release -maxdepth 1 -type f -executable)
SCRIPTS="scripts start.sh stop.sh watchdog.sh"

rsync -urP $SCRIPTS vps:~/zkbinfo/
rsync -uP $BINARIES vps:~/zkbinfo/bin/
rsync -urP public vps:~/zkbinfo/