#!/bin/bash



CURRENT=${1:-$(date +'%Y-%m-%d' -d "7 day ago")}
FINISH=${2:-$(date +'%Y-%m-%d' -d "1 day ago")}
TODAY=$(date +'%Y-%m-%d')

if [ "$CURRENT" != "$TODAY" ]; then
    while [ "$CURRENT" != "$FINISH" ]; do
      echo $CURRENT
      ZKBINFO_HOST=$(hostname -I | awk '{print $1}') ~/zkbinfo/bin/killmail $CURRENT
      CURRENT=$(date -I -d "$CURRENT + 1 day")
    done
fi