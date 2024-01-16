-- Add migration script here
CREATE TABLE users (
  id uuid NOT NULL,
  username TEXT UNIQUE NOT NULL,
  password TEXT NOT NULL,
  email TEXT UNIQUE NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ,
  last_login TIMESTAMPTZ 
);

insert into users (id, username, email,password)
values ('1224560f-4d62-4eb0-8dd0-488f73871c87'

, 'ferris','a@a.com', '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw');