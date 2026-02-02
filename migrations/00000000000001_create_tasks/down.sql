-- Drop trigger first
DROP TRIGGER IF EXISTS update_tasks_updated_at ON tasks;

-- Drop function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop table (indexes are dropped automatically)
DROP TABLE IF EXISTS tasks;
