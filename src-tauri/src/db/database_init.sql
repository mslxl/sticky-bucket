CREATE TABLE IF NOT EXISTS package(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  author TEXT,
  upstream TEXT
);

INSERT OR REPLACE INTO package(id, name) VALUES(0, "Inbox");

CREATE TABLE IF NOT EXISTS tag(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  key TEXT NOT NULL,
  name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS meme(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  type TEXT NOT NULL,
  content TEXT NOT NULL,
  create_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  update_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  fav INTEGER NOT NULL DEFAULT 0,
  trash INTEGER NOT NULL DEFAULT 0,
  pkg_id NOT NULL DEFAULT 0,
  CONSTRAINT pkg_fk 
    FOREIGN KEY (pkg_id)
    REFERENCES package(id)
);

CREATE TABLE IF NOT EXISTS meme_tag(
  tag_id INTEGER NOT NULL,
  meme_id INTEGER NOT NULL,
  CONSTRAINT table_name_pk PRIMARY KEY(tag_id,meme_id),
	CONSTRAINT table_name_meme_id_fk FOREIGN KEY(meme_id) REFERENCES meme(id),
	CONSTRAINT table_name_tag_id_fk FOREIGN KEY(tag_id) REFERENCES tag(id)
);

CREATE TRIGGER [UpdateUpdateTime] AFTER UPDATE ON meme FOR EACH ROW
BEGIN
    UPDATE meme SET update_time = CURRENT_TIMESTAMP WHERE id = OLD.id;
END