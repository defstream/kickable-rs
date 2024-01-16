export PROTOCOL=http
export ADDR=0.0.0.0
export URL="$PROTOCOL://$ADDR"
wrk -c 400 -t 10 -d 30s http://axum.kickable.orb.local:8080/it
wrk -c 400 -t 10 -d 30s http://gotham.kickable.orb.local:8080/it
wrk -c 400 -t 10 -d 30s http://graphul.kickable.orb.local:8080/it
wrk -c 400 -t 10 -d 30s http://poem.kickable.orb.local.local:8080/it
wrk -c 400 -t 10 -d 30s http://rocket.kickable.orb.local:8080/it
wrk -c 400 -t 10 -d 30s http://rouille.kickable.orb.local:8080/it
wrk -c 400 -t 10 -d 30s http://viz.kickable.orb.local:8080/it
wrk -c 400 -t 10 -d 30s http://warp.kickable.orb.local:8080/it
