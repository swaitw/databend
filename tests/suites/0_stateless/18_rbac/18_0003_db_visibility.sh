#!/usr/bin/env bash

CURDIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
. "$CURDIR"/../../../shell_env.sh

export TEST_USER_NAME="owner"
export TEST_USER_PASSWORD="password"
export TEST_USER_CONNECT="bendsql --user=owner --password=password --host=${QUERY_MYSQL_HANDLER_HOST} --port ${QUERY_HTTP_HANDLER_PORT}"


echo "drop role if exists role1" | $BENDSQL_CLIENT_CONNECT
echo "drop role if exists role2" | $BENDSQL_CLIENT_CONNECT
echo "drop user if exists u1" | $BENDSQL_CLIENT_CONNECT
echo "drop user if exists u2" | $BENDSQL_CLIENT_CONNECT
echo "drop user if exists u3" | $BENDSQL_CLIENT_CONNECT
echo "drop database if exists db1" | $BENDSQL_CLIENT_CONNECT
echo "drop database if exists db2" | $BENDSQL_CLIENT_CONNECT
echo "drop database if exists db_u3" | $BENDSQL_CLIENT_CONNECT
echo "drop database if exists db_root" | $BENDSQL_CLIENT_CONNECT
echo "create database db_root" | $BENDSQL_CLIENT_CONNECT
echo "create table db_root.t1(id int)" | $BENDSQL_CLIENT_CONNECT
echo "insert into db_root.t1 values(4)" | $BENDSQL_CLIENT_CONNECT
echo "create role role1;" | $BENDSQL_CLIENT_CONNECT
echo "create role role2;" | $BENDSQL_CLIENT_CONNECT
echo "grant create database on *.* to role role1;" | $BENDSQL_CLIENT_CONNECT
echo "grant create database on *.* to role role2;" | $BENDSQL_CLIENT_CONNECT
echo "create user u1 identified by '123' with DEFAULT_ROLE='role1';" | $BENDSQL_CLIENT_CONNECT
echo "create user u2 identified by '123' with DEFAULT_ROLE='role1';" | $BENDSQL_CLIENT_CONNECT
echo "create user u3 identified by '123' with DEFAULT_ROLE='role2';" | $BENDSQL_CLIENT_CONNECT

echo "=== test u1 with role1 ==="
echo "grant role role1 to u1;" | $BENDSQL_CLIENT_CONNECT
export TEST_U1_CONNECT="bendsql --user=u1 --password=123 --host=${QUERY_MYSQL_HANDLER_HOST} --port ${QUERY_HTTP_HANDLER_PORT}"
echo "show databases" | $TEST_U1_CONNECT
echo "create database db1;" | $TEST_U1_CONNECT
echo "grant delete on db1.* to u1" | $BENDSQL_CLIENT_CONNECT
echo "drop database db1;" | $TEST_U1_CONNECT
echo "select count(name)>0, count(dropped_on is not null)>0 from system.databases_with_history where name='db1'" | $BENDSQL_CLIENT_CONNECT
echo "select name, dropped_on is not null from system.databases_with_history where name='db1'" | $TEST_U1_CONNECT
echo "select name from system.databases_with_history where name!='db1'" | $TEST_U1_CONNECT
echo "create database db1;" | $TEST_U1_CONNECT
echo "create table db1.t1(id int);" | $TEST_U1_CONNECT
echo "insert into db1.t1 values(1);" | $TEST_U1_CONNECT
echo "select * from db1.t1;" | $TEST_U1_CONNECT
echo "select * from db_root.t1;" | $TEST_U1_CONNECT
echo "show databases" | $TEST_U1_CONNECT

echo "=== test u2 with role1 ==="
echo "grant role role1 to u2;" | $BENDSQL_CLIENT_CONNECT
export TEST_U2_CONNECT="bendsql --user=u2 --password=123 --host=${QUERY_MYSQL_HANDLER_HOST} --port ${QUERY_HTTP_HANDLER_PORT}"
echo "show databases" | $TEST_U2_CONNECT
echo "create database db2" | $TEST_U2_CONNECT
echo "create table db2.t2(id int);" | $TEST_U2_CONNECT
echo "insert into db2.t2 values(2);" | $TEST_U2_CONNECT
echo "show databases" | $TEST_U2_CONNECT
echo "select * from db2.t2;" | $TEST_U2_CONNECT
echo "select * from db1.t1;" | $TEST_U2_CONNECT
echo "select * from db2.t2;" | $TEST_U1_CONNECT

echo "=== test u3 with role2 ==="
echo "grant role role2 to u3;" | $BENDSQL_CLIENT_CONNECT
export TEST_U3_CONNECT="bendsql --user=u3 --password=123 --host=${QUERY_MYSQL_HANDLER_HOST} --port ${QUERY_HTTP_HANDLER_PORT}"

echo "show databases" | $TEST_U3_CONNECT
echo "create database db_u3" | $TEST_U3_CONNECT
echo "create table db_u3.t3(id int)" | $TEST_U3_CONNECT
echo "insert into db_u3.t3 values(3)" | $TEST_U3_CONNECT
echo "show databases" | $TEST_U3_CONNECT
echo "select * from db1.t1" | $TEST_U3_CONNECT
echo "select * from db2.t2" | $TEST_U3_CONNECT
echo "select * from db_root.t1" | $TEST_U3_CONNECT
echo "select * from db_u3.t3" | $TEST_U3_CONNECT

echo "=== test u3 with role2 and role1 secondary roles all ==="
echo "grant role role1 to u3" | $BENDSQL_CLIENT_CONNECT
echo "SET SECONDARY ROLES ALL; show databases" | $TEST_U3_CONNECT
echo "SET SECONDARY ROLES ALL; select * from db1.t1" | $TEST_U3_CONNECT
echo "SET SECONDARY ROLES ALL; select * from db2.t2" | $TEST_U3_CONNECT
echo "SET SECONDARY ROLES ALL; select * from db_root.t1" | $TEST_U3_CONNECT
echo "SET SECONDARY ROLES ALL; select * from db_u3.t3" | $TEST_U3_CONNECT

echo "=== test u3(set role1) with role2 and role1 secondary roles none ==="
echo "set role role1; SET SECONDARY ROLES NONE; show databases;" | $TEST_U3_CONNECT
echo "set role role1; SET SECONDARY ROLES NONE; select * from db1.t1" | $TEST_U3_CONNECT
echo "set role role1; SET SECONDARY ROLES NONE; select * from db2.t2" | $TEST_U3_CONNECT
echo "set role role1; SET SECONDARY ROLES NONE; select * from db_root.t1" | $TEST_U3_CONNECT
echo "set role role1; SET SECONDARY ROLES NONE; select * from db_u3.t3" | $TEST_U3_CONNECT

echo "=== test root user ==="
echo "show databases" | $BENDSQL_CLIENT_CONNECT
echo "select * from db1.t1" | $BENDSQL_CLIENT_CONNECT
echo "select * from db2.t2" | $BENDSQL_CLIENT_CONNECT
echo "select * from db_u3.t3" | $BENDSQL_CLIENT_CONNECT
echo "select * from db_root.t1" | $BENDSQL_CLIENT_CONNECT

echo "=== test system.tables ==="
echo "drop user if exists a;" | $BENDSQL_CLIENT_CONNECT
echo "drop user if exists b;" | $BENDSQL_CLIENT_CONNECT
echo "drop role if exists b;" | $BENDSQL_CLIENT_CONNECT
echo "drop role if exists a;" | $BENDSQL_CLIENT_CONNECT
echo "drop database if exists a;" | $BENDSQL_CLIENT_CONNECT
echo "create user a identified by '123' with default_role='a'" | $BENDSQL_CLIENT_CONNECT
echo "create role a" | $BENDSQL_CLIENT_CONNECT
echo "create database a" | $BENDSQL_CLIENT_CONNECT
echo "grant ownership on a.* to role a" | $BENDSQL_CLIENT_CONNECT
echo "grant role a to a" | $BENDSQL_CLIENT_CONNECT
echo "create table a.b(id int)" | $BENDSQL_CLIENT_CONNECT
echo "create role b" | $BENDSQL_CLIENT_CONNECT
echo "grant ownership on a.b to role b" | $BENDSQL_CLIENT_CONNECT
export TEST_A_CONNECT="bendsql --user=a --password=123 --host=${QUERY_MYSQL_HANDLER_HOST} --port ${QUERY_HTTP_HANDLER_PORT}"
echo "select name, owner from system.tables where database = 'a'and name = 'b'" | $TEST_A_CONNECT
echo "drop user if exists a;" | $BENDSQL_CLIENT_CONNECT
echo "drop user if exists b;" | $BENDSQL_CLIENT_CONNECT
echo "drop role if exists b;" | $BENDSQL_CLIENT_CONNECT
echo "drop role if exists a;" | $BENDSQL_CLIENT_CONNECT
echo "drop database if exists a;" | $BENDSQL_CLIENT_CONNECT
