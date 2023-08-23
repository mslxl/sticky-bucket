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
  value TEXT NOT NULL
);


CREATE TABLE IF NOT EXISTS meme(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL, /* 名字 */
  description TEXT, /* 对表情的细节描述 */
  ty TEXT NOT NULL, /* 类型，目前取值 image 或 text */
  hash TEXT NOT NULL, /* 内容 hash */
  create_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  update_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  fav INTEGER NOT NULL DEFAULT 0,
  trash INTEGER NOT NULL DEFAULT 0,
  pkg_id INTEGER NOT NULL DEFAULT 0,
  parent INTEGER,
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