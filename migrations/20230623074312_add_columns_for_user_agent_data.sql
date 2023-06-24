ALTER TABLE visitors DROP COLUMN user_agent;

ALTER TABLE visitors
ADD COLUMN user_agent varchar(255) NULL,
  ADD COLUMN user_agent_device varchar(255) NULL,
  ADD COLUMN user_agent_os varchar(255) NULL;