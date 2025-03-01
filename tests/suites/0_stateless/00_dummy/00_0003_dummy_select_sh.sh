#!/usr/bin/env bash

CURDIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
. "$CURDIR"/../../../shell_env.sh


echo "SELECT 1;select 2;select 3;" | $BENDSQL_CLIENT_CONNECT

echo "SELECT 1" | $BENDSQL_CLIENT_CONNECT
echo "SELECT 2" | $BENDSQL_CLIENT_CONNECT
echo "SELECT 3" | $BENDSQL_CLIENT_CONNECT
