CREATE TABLE IF NOT EXISTS guilds (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  guild_id TEXT NOT NULL,
  guild_name TEXT NOT NULL,
  default_role TEXT NOT NULL,
);

CREATE TABLE IF NOT EXISTS commands (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  guild_id TEXT NOT NULL,
  command_name TEXT NOT NULL,
  required_role TEXT NOT NULL,
  FOREIGN KEY (guild_id) REFERENCES guilds (guild_id)
);
