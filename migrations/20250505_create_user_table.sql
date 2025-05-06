CREATE TABLE IF NOT EXISTS user
(
  id          INTEGER NOT NULL PRIMARY KEY,
  name       VARCHAR,
  password   VARCHAR
);

INSERT INTO user (name, password) VALUES ("admin", "admin");