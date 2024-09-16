#!/bin/bash

export ZKBINFO_HOST=185.87.51.139
export ZKBINFO_PORT=8080

export ZKB_CLI="/home/seb/zkbinfo/bin/zkb_client"
export ZKBINFO="/home/seb/zkbinfo/bin/zkbinfo"

if ! pgrep -f "$ZKBINFO" > /dev/null; then
  echo "$(date): Процесс $ZKBINFO не найден. Выход..."
  exit 0
fi

if ! pgrep -f "$ZKB_CLI" > /dev/null; then
  echo "$(date): Процесс $ZKB_CLI не найден. Запускаем..."
  $CMD
else
  echo "$(date): Процесс $ZKB_CLI уже запущен."
fi

