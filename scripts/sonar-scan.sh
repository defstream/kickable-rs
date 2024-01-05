docker run \
        --rm \
        --net host \
        -e SONAR_HOST_URL=$SONAR_HOST_URL \
        -v ${PWD}:/usr/src  \
        sonarsource/sonar-scanner-cli \
    -Dsonar.projectKey=kickable \
    -Dsonar.sources=. \
    -Dsonar.host.url=$SONAR_HOST_URL \
    -Dsonar.token=$SONAR_TOKEN