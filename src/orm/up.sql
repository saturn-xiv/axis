CREATE TABLE jobs(
    id INTEGER PRIMARY KEY NOT NULL,
    cid VARCHAR(36) NOT NULL,
    name VARCHAR(255) NOT NULL,
    host VARCHAR(255) NOT NULL,
    reason TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_jobs_cid ON jobs(cid);

CREATE INDEX idx_jobs_name ON jobs(name);

CREATE INDEX idx_jobs_host ON jobs(host);

CREATE UNIQUE INDEX idx_jobs_cid_host ON jobs(cid, host);

CREATE TABLE logs(
    id INTEGER PRIMARY KEY NOT NULL,
    host VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_logs_host ON logs(host);