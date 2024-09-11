#!/bin/bash

export ZKBINFO_HOST=185.87.51.139
export ZKBINFO_PORT=8080
export PROCESS_NAME="zkb_client"
export CMD="/home/seb/zkbinfo/bin/${PROCESS_NAME}"

if ! pgrep -f "$CMD" > /dev/null; then
  echo "$(date): Процесс $PROCESS_NAME не найден. Запускаем..."
  $CMD
else
  echo "$(date): Процесс $PROCESS_NAME уже запущен."
fi

