CREATE TABLE taggroups (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  ordering SMALLINT NOT NULL
);

CREATE TABLE tags (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  map TEXT DEFAULT NULL,
  taggroup_id integer REFERENCES taggroups(id)
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE usergroups (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE notes (
  id SERIAL PRIMARY KEY,
  owner_id integer NOT NULL REFERENCES users(id)
);

CREATE TABLE notetexts (
  id SERIAL PRIMARY KEY,
  author_id integer NOT NULL REFERENCES users(id),
  note_id integer NOT NULL REFERENCES notes(id),
  text TEXT,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- HABTM tables

CREATE TABLE notestags (
  note_id integer NOT NULL REFERENCES notes(id),
  tag_id integer NOT NULL REFERENCES tags(id),
  PRIMARY KEY (note_id, tag_id)
);

CREATE TABLE usersusergroups (
  user_id integer NOT NULL REFERENCES users(id),
  usergroup_id integer NOT NULL REFERENCES usergroups(id),
  PRIMARY KEY (user_id, usergroup_id)
);

CREATE TABLE notesusers (
  note_id integer NOT NULL REFERENCES notes(id),
  user_id integer NOT NULL REFERENCES users(id),
  PRIMARY KEY (note_id, user_id)
);

CREATE TABLE notesusergroups (
  note_id integer NOT NULL REFERENCES notes(id),
  usergroup_id integer NOT NULL REFERENCES usergroups(id),
  PRIMARY KEY (note_id, usergroup_id)
);
