CREATE TABLE agents(
  id INTEGER PRIMARY KEY NOT NULL,
  mac VARCHAR(17) NOT NULL, 
  ip VARCHAR(45) NOT NULL, 
  name VARCHAR(255) NOT NULL, 
  updated_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_agents_mac ON agents(mac);
CREATE INDEX idx_agents_ip ON agents(ip);
CREATE INDEX idx_agents_name ON agents(name);

CREATE TABLE logs(
  id INTEGER PRIMARY KEY NOT NULL,      
  mac VARCHAR(17) NOT NULL, 
  ip VARCHAR(45) NOT NULL,  
  task VARCHAR(255) NOT NULL,
  message TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_logs_task ON logs(task);
CREATE INDEX idx_logs_ip ON logs(ip);
CREATE INDEX idx_logs_mac ON logs(mac);
