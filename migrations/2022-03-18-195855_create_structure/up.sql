-- Your SQL goes here
-- This file should undo anything in `up.sql`
create table users (
    id varchar primary key not null,
    name text unique not null,
    email text unique not null,

    created_at text not null,

    password varchar not null,
    password_salt varchar not null,

    is_banned boolean not null,
    role text check(role in ('admin', 'user')) not null
);

create table blog_posts (
    id varchar primary key not null,
    title text unique not null,
    brief text not null,
    contents text not null,

    author_id varchar not null,

    created_at text not null,
    updated_at text not null,

    visibility text check(visibility in ('all', 'authenticated')) not null,

    foreign key (author_id) references users(id)
);

create table comments (
    id varchar primary key not null,
    contents text not null,

    author_id varchar not null,
    post_id varchar not null,
    reply_to_id varchar,

    created_at text not null,
    updated_at text not null,

    is_deleted boolean not null,

    foreign key (author_id) references users(id),
    foreign key (post_id) references blog_posts(id),
    foreign key (reply_to_id) references comments(id)
);

create table projects (
    id varchar primary key not null,
    title text unique not null,
    brief text not null,

    author_id varchar not null,

    visibility text check(visibility in ('all', 'authenticated')) not null,

    foreign key (author_id) references users(id)
);

create table project_editor_junctions (
    project_id varchar not null,
    user_id varchar not null,

    constraint pk primary key (
        project_id, user_id
    ),

    foreign key (project_id) references projects(id),
    foreign key (user_id) references users(id)
);

create table project_blog_post_junctions (
    project_id varchar not null,
    post_id varchar not null,

    constraint pk primary key (
        project_id, post_id
    ),

    foreign key (project_id) references projects(id),
    foreign key (post_id) references blog_posts(id)
);