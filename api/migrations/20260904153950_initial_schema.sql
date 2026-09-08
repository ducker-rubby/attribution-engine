-- TODO: Add indexes for queries
CREATE TABLE link_group (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name TEXT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE link (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name TEXT NOT NULL,
  redirect_url TEXT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE link_link_group (
  link_id INT REFERENCES link (id) ON DELETE CASCADE,
  link_group_id INT REFERENCES link_group (id) ON DELETE CASCADE,
  PRIMARY KEY (link_id, link_group_id)
);


CREATE TABLE country (
  id CHAR(2) PRIMARY KEY,
  full_name TEXT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE referrer (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  url TEXT UNIQUE NOT NULL
);


CREATE TABLE user_agent (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  agent TEXT UNIQUE NOT NULL
);


CREATE TABLE click (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  link_id INT NOT NULL REFERENCES link (id) ON DELETE CASCADE,
  ip INET,
  country_id CHAR(2) REFERENCES country (id),
  user_agent_id INT REFERENCES user_agent (id),
  referrer_id INT REFERENCES referrer (id),
  -- 0 = UNKNOWN, 1=DESKTOP, 2=MOBILE, 3=TABLE, 4=BOT
  device_type SMALLINT NOT NULL DEFAULT 0,
  is_bot BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT chk_device_type CHECK (device_type BETWEEN 0 AND 4)
);


CREATE INDEX idx_click_link_id ON click (link_id, created_at);


CREATE INDEX idx_click_user_agent_id ON click (user_agent_id);


CREATE INDEX idx_click_referrer_id ON click (referrer_id);
