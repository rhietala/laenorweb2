-- This file should undo anything in `up.sql`

DELETE FROM tags WHERE taggroup_id = uuid_generate_v5 (uuid_nil(), 'laenor');
DELETE FROM tags WHERE taggroup_id = uuid_generate_v5 (uuid_nil(), 'rothikgen');
DELETE FROM tags WHERE taggroup_id = uuid_generate_v5 (uuid_nil(), 'lucentium');
DELETE FROM tags WHERE taggroup_id = uuid_generate_v5 (uuid_nil(), 'furnachia');
DELETE FROM tags WHERE taggroup_id = uuid_generate_v5 (uuid_nil(), 'desolathya');
