CREATE TABLE users(
  id INTEGER PRIMARY KEY NOT NULL,  
  name VARCHAR(255) NOT NULL,
  password BLOB NOT NULL,
  uid VARCHAR(36) NOT NULL,  
  sign_in_count BIGINT NOT NULL DEFAULT 0,
  current_sign_in_at TIMESTAMP,
  current_sign_in_ip VARCHAR(39),
  last_sign_in_at TIMESTAMP,
  last_sign_in_ip VARCHAR(39),  
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);
CREATE UNIQUE INDEX idx_users_name ON users(name);
CREATE UNIQUE INDEX idx_users_uid ON users(uid);

CREATE TABLE logs(
  id INTEGER PRIMARY KEY NOT NULL,  
  type VARCHAR(255) NOT NULL,
  message TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_logs_type ON logs(type);

CREATE TABLE settings(
  id INTEGER PRIMARY KEY NOT NULL,
  key VARCHAR(255) NOT NULL,
  value BLOB NOT NULL,
  salt BLOB,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);
CREATE UNIQUE INDEX idx_settings_key ON settings(key);


