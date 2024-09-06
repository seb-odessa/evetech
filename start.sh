#!/bin/bash

ARCH=logs-$(date +"%Y%m%d_%H%M%S").tar.gz
tar --remove-files -czf $ARCH logs
echo logs archived: $ARCH

IP=$(hostname -I | awk '{print $1}')
echo Service ip: $IP

export ZKBINFO_HOST=$IP
export ZKBINFO_PORT=8080
export ZKBGUI_HOST=$IP
export ZKBGUI_PORT=8088

mkdir -p logs

nohup ~/zkbinfo/bin/zkbinfo > ~/zkbinfo/logs/zkbinfo.log&
echo zkbinfo started...
sleep 1
head ~/zkbinfo/logs/zkbinfo.log

#nohup ~/zkbinfo/bin/zkbgui > ~/zkbinfo/logs/zkbgui.log&
#echo zkbgui started...
#sleep 1
#head ~/zkbinfo/logs/zkbgui.log

nohup ~/zkbinfo/bin/websocket_client > ~/zkbinfo/logs/websocket_client.log&
echo websocket_client started...
sleep 1
head ~/zkbinfo/logs/websocket_client.log

