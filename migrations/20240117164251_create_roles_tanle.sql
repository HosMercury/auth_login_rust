-- Create `groups` table.
create table groups (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

-- Create `permissions` table.
create table permissions (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);


-- # Join tables.

-- Create `users_groups` table for many-to-many relationships between users and groups.
create table users_groups (
    user_id uuid NOT NULL REFERENCES users(id),
    group_id INTEGER REFERENCES groups(id),
    PRIMARY KEY (user_id, group_id)
);

-- Create `groups_permissions` table for many-to-many relationships between groups and permissions.
create table groups_permissions (
    group_id INTEGER REFERENCES groups(id),
    permission_id INTEGER REFERENCES permissions(id),
    PRIMARY KEY (group_id, permission_id)
);


-- Insert "users" and "superusers" groups.
INSERT INTO groups (name) values ('users');
INSERT INTO groups (name) values ('superusers');

-- Insert individual permissions.
INSERT INTO permissions (name) values ('protected.read');
INSERT INTO permissions (name) values ('restricted.read');

-- Insert group permissions.
INSERT INTO groups_permissions (group_id, permission_id)
values (
    (select id from groups where name = 'users'),
    (select id from permissions where name = 'protected.read')
), (
    (select id from groups where name = 'superusers'),
    (select id from permissions where name = 'restricted.read')
);

-- Insert users into groups.
insert into users_groups (user_id, group_id)
values (
    (select id from users where username = 'ferris'),
    (select id from groups where name = 'users')
), (
    (select id from users where username = 'admin'),
    (select id from groups where name = 'users')
), (
    (select id from users where username = 'admin'),
    (select id from groups where name = 'superusers')
);