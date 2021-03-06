INSERT INTO users (id, name) VALUES
  (1, 'astrax'),
  (2, 'ruska');

INSERT INTO notes (id, owner_id) VALUES
  (1, 1),
  (2, 2);

INSERT INTO notetexts (id, author_id, note_id, text) VALUES
  (1,
   1,
   1,
   E'Entrance from outworld with chant zonni\n\n                  4\n                 /\n      ?   ?     o    dwarf miners\n       \\ /      |\\\ngoblins o-o-o   o o-o\n             \\ /\n              o\n              |\n    3-o-o-o-o-o-o-o-o-o-3\n              |\n              o\n              |\n              o     ?\n              |     |\n              2     5                 2-o-6\n              |     |                /\n    1 <-u/d-> 1     o-o-o-o-o-o-o-o-o\n    |                       |       |\n    o         ^             o-o-o-o-o\n    |         |             |        \\\n    o         u/d           o         2-o-7\n    |           \\            \\\n    o            ------>      1\n    |\n    o\n    |\n    o\n    |\n    E\n\nE: Entrance\n2: forcefield, newbie level limit (lvl 34 ok)\n3: out to Arelium\n4: carousel, "push button" and then exit in some direction, four possible exits\n5: blocking golem\n6: Klodge the newbie-eating demon\n7: room with booze\n\n\nCarousel exits\n\nA: one room with locked door, "climb ladder" for exit to arelium\nB: smallish maze with spiders\nC: three rooms with bats\n'),
  (2,
   2,
   2,
   E'\n    out-o-o-o   \n    | | | |\n    o-o-o-u--u/d--d/slide\n    | | | |\n    T-o-S-o\n    | | | |\n    o-o-o-o\n\n  T= tree\n  S= swings\n  u= up \n  d= down\n  slide= brings you back to ground lvl (the first u from the left)\n\n\n\nNotes:\n\n  * Be sure to climb the tree in the southwestern corner.  Youll need a key\n    to get through the locked door.  Wonder where you can get a key from?\n\n  * Arnolds one tough dude.\n\n  * Swing on swings.\n  \n  * Map and info by Quenthalion.');

INSERT INTO notestags (note_id, tag_id) VALUES
  (1, (select id from tags where name = 'Arelium')),
  (2, (select id from tags where name = 'Arelium')),
  (2, (select id from tags where name = 'Newbie playgrounds'));

INSERT INTO notesusers (note_id, user_id) VALUES
  (1, 1),
  (1, 2),
  (2, 1);

INSERT INTO usergroups (id, name) VALUES
  (1, 'svoima');

INSERT INTO usersusergroups (user_id, usergroup_id) VALUES
  (1, 1);

INSERT INTO notesusergroups (note_id, usergroup_id) VALUES
  (1, 1);
