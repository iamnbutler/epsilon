CREATE TABLE IF NOT EXISTS day_tasks (
  id SERIAL PRIMARY KEY,
  day_id INTEGER REFERENCES days (id),
  task_id INTEGER REFERENCES tasks (id),
  UNIQUE (day_id, task_id)
);

ALTER TABLE tasks ADD COLUMN day_id INTEGER REFERENCES days (id);
