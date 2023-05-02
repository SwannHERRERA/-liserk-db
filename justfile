serve:
  cargo run --bin liserk-server

create_database DB_NAME:
  http POST localhost:3000/database/{{DB_NAME}}
