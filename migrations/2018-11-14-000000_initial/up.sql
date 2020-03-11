
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE taggroups (
  id uuid NOT NULL DEFAULT uuid_generate_v4 (),
  name TEXT NOT NULL,
  ordering SMALLINT NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE tags (
  id uuid NOT NULL DEFAULT uuid_generate_v4 (),
  name TEXT NOT NULL,
  map TEXT DEFAULT NULL,
  taggroup_id uuid REFERENCES taggroups(id),
  PRIMARY KEY (id)
);

CREATE TABLE users (
  id uuid NOT NULL DEFAULT uuid_generate_v4 (),
  name TEXT NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE usergroups (
  id uuid NOT NULL DEFAULT uuid_generate_v4 (),
  name TEXT NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE notes (
  id uuid NOT NULL DEFAULT uuid_generate_v4 (),
  owner_id uuid NOT NULL REFERENCES users(id),
  PRIMARY KEY (id)
);

CREATE TABLE notetexts (
  id uuid NOT NULL DEFAULT uuid_generate_v4 (),
  author_id uuid NOT NULL REFERENCES users(id),
  note_id uuid NOT NULL REFERENCES notes(id),
  text TEXT,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id)
);

-- HABTM tables

CREATE TABLE notestags (
  note_id uuid NOT NULL REFERENCES notes(id),
  tag_id uuid NOT NULL REFERENCES tags(id),
  PRIMARY KEY (note_id, tag_id)
);

CREATE TABLE usersusergroups (
  user_id uuid NOT NULL REFERENCES users(id),
  usergroup_id uuid NOT NULL REFERENCES usergroups(id),
  PRIMARY KEY (user_id, usergroup_id)
);

CREATE TABLE notesusers (
  note_id uuid NOT NULL REFERENCES notes(id),
  user_id uuid NOT NULL REFERENCES users(id),
  PRIMARY KEY (note_id, user_id)
);

CREATE TABLE notesusergroups (
  note_id uuid NOT NULL REFERENCES notes(id),
  usergroup_id uuid NOT NULL REFERENCES usergroups(id),
  PRIMARY KEY (note_id, usergroup_id)
);
