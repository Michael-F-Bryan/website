CREATE TABLE timesheet_entries (
    id SERIAL PRIMARY KEY,
    user_id INTEGER,
    start_time TIMESTAMP,
    end_time TIMESTAMP,
    breaks FLOAT,
    morning TEXT,
    afternoon TEXT
);

CREATE INDEX ix_ts_entries_user_id ON timesheet_entries (user_id);