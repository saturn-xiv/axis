CREATE TABLE agents(
  id INTEGER PRIMARY KEY NOT NULL,
  mac VARCHAR(17) NOT NULL, 
  ip VARCHAR(45) NOT NULL,
  name VARCHAR(255) NOT NULL,
  hardware TEXT NOT NULL,
  os TEXT NOT NULL,  
  version TEXT,
  online BOOLEAN NOT NULL DEFAULT FALSE,  
  updated_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_agents_mac ON agents(mac);
CREATE INDEX idx_agents_ip ON agents(ip);
CREATE INDEX idx_agents_name ON agents(name);

CREATE TABLE groups(
  id INTEGER PRIMARY KEY NOT NULL,  
  name VARCHAR(255) NOT NULL,  
  updated_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_groups_name ON groups(name);

CREATE TABLE groups_agents(
  id INTEGER PRIMARY KEY NOT NULL,  
  group_id INTEGER NOT NULL,  
  agent_id INTEGER NOT NULL,  
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_groups_agents ON groups_agents(group_id, agent_id);

CREATE TABLE logs(
  id INTEGER PRIMARY KEY NOT NULL,    
  agent_id INTEGER NOT NULL,
  ip VARCHAR(45) NOT NULL,  
  task VARCHAR(255) NOT NULL,
  message TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_logs_task ON logs(task);
CREATE INDEX idx_logs_ip ON logs(ip);
