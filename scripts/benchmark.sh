export PROTOCOL=http
export ADDR=0.0.0.0
export URL="$PROTOCOL://$ADDR"
 wrk -c 400 -t 10 -d 30s $URL:8001/it
 wrk -c 400 -t 10 -d 30s $URL:8002/it
 wrk -c 400 -t 10 -d 30s $URL:8003/it
 wrk -c 400 -t 10 -d 30s $URL:8004/it
 wrk -c 400 -t 10 -d 30s $URL:8005/it
 wrk -c 400 -t 10 -d 30s $URL:8006/it
 wrk -c 400 -t 10 -d 30s $URL:8007/it
 wrk -c 400 -t 10 -d 30s $URL:8008/it
 wrk -c 400 -t 10 -d 30s $URL:8009/it
 wrk -c 400 -t 10 -d 30s $URL:8010/it
