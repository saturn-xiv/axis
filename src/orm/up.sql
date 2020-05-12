CREATE TABLE IF NOT EXISTS logs(
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    uid VARCHAR(17) NOT NULL,
    host VARCHAR(255) NOT NULL,
    job VARCHAR(255) NOT NULL,
    task VARCHAR(255) NOT NULL,
    result TEXT,
    updated TIMESTAMP NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_logs_uid ON logs(uid);

CREATE INDEX idx_logs_host ON logs(host);

CREATE INDEX idx_logs_job ON logs(job);

CREATE INDEX idx_logs_task ON logs(task);