CREATE TYPE repeat_frequency AS ENUM ('daily', 'weekly', 'biweekly', 'monthly', 'yearly');

CREATE TABLE IF NOT EXISTS
    rituals (
        id SERIAL PRIMARY KEY,
        title TEXT NOT NULL,
        frequency repeat_frequency,
        active BOOLEAN NOT NULL DEFAULT TRUE
    );
