A simple database interaction implementation using actix-web framework and diesel using sqlite Db.

Database setup using diesel-cli
==============================
cargo install diesel_cli --no-default-features --features sqlite

echo DATABASE_URL=test.db > .env

diesel setup

diesel migration generate create_users

diesel migration run

Get/Post requests for setting/fetching a user
=============================================
Format <user_name: String/id: Uuid>

POST
curl -x POST --url http://127.0.0.1:8080/user -H 'Content-type: application/json' -d {"name":"<Some-user-name>"}

GET
curl -x GET --url http://127.0.0.1:8080/user/<id>
where id is the unique id returned in POST json


