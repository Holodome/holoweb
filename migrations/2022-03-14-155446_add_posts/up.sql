-- Your SQL goes here
create table blog_posts (
    id varchar primary key not null,
    name text unique not null,
    contents text not null
)