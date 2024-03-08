CREATE TABLE IF NOT EXISTS day_rituals (
    day_id INTEGER NOT NULL,
    ritual_id INTEGER NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (day_id, ritual_id),
    FOREIGN KEY (day_id) REFERENCES days(id),
    FOREIGN KEY (ritual_id) REFERENCES rituals(id)
);
