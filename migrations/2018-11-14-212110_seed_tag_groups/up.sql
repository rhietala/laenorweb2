-- Your SQL goes here

INSERT INTO taggroups (id, name, ordering) VALUES
  (uuid_generate_v5 (uuid_nil(), 'laenor'), 'Laenor areas', 10),
  (uuid_generate_v5 (uuid_nil(), 'rothikgen'), 'Rothikgen areas', 20),
  (uuid_generate_v5 (uuid_nil(), 'lucentium'), 'Lucentium areas', 30),
  (uuid_generate_v5 (uuid_nil(), 'furnachia'), 'Furnachia areas', 40),
  (uuid_generate_v5 (uuid_nil(), 'desolathya'), 'Desolathya areas', 50),
  (uuid_generate_v5 (uuid_nil(), 'other_area'), 'Other areas', 60),
  (uuid_generate_v5 (uuid_nil(), 'monster'), 'Monsters', 70),
  (uuid_generate_v5 (uuid_nil(), 'quest'), 'Quests', 80),
  (uuid_generate_v5 (uuid_nil(), 'other'), 'Others', 90);
