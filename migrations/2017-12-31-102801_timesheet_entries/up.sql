CREATE TABLE timesheet_entries (
    id SERIAL PRIMARY KEY,
    user_id INTEGER,
    start_time TIMESTAMP,
    end_time TIMESTAMP,
    breaks FLOAT,
    morning TEXT,
    afternoon TEXT,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX ix_ts_entries_user_id ON timesheet_entries (user_id);
SELECT diesel_manage_updated_at('timesheet_entries');