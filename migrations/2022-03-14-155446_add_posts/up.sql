-- Your SQL goes here
create table posts (
    id varchar primary key not null,
    name text unique not null,
    contents text not null
)