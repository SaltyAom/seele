#!/bin/sh
./meilisearch --http-payload-size-limit 1048576000 --no-analytics > /dev/null 2>&1 &

./seele
