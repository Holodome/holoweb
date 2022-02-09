-- Your SQL goes here
create table users (
   id serial not null primary key,
   name text not null ,
   email text not null ,
   created_at timestamp not null
)
