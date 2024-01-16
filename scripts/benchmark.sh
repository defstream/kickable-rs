export PROTOCOL=http
export ADDR=0.0.0.0
export PORT=8080
export URL="$PROTOCOL://$ADDR"
wrk -c 400 -t 10 -d 30s $PROTOCOL://axum.kickable.orb.local:$PORT/it
wrk -c 400 -t 10 -d 30s $PROTOCOL://gotham.kickable.orb.local:$PORT/it
wrk -c 400 -t 10 -d 30s $PROTOCOL://graphul.kickable.orb.local:$PORT/it
wrk -c 400 -t 10 -d 30s $PROTOCOL://poem.kickable.orb.local.local:$PORT/it
wrk -c 400 -t 10 -d 30s $PROTOCOL://rocket.kickable.orb.local:$PORT/it
wrk -c 400 -t 10 -d 30s $PROTOCOL://rouille.kickable.orb.local:$PORT/it
wrk -c 400 -t 10 -d 30s $PROTOCOL://viz.kickable.orb.local:$PORT/it
wrk -c 400 -t 10 -d 30s $PROTOCOL://warp.kickable.orb.local:$PORT/it
