CREATE TABLE users (
    id uuid NOT NULL,
    PRIMARY KEY(id),
    email VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(36) NOT NULL,
    password TEXT NOT NULL,
    profile_photo TEXT,
    failed_attempts SMALLINT NOT NULL DEFAULT 0,
    email_confirmed BOOLEAN NOT NULL DEFAULT false,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz
);

CREATE TABLE confirmation_tokens(
    confirmation_token TEXT NOT NULL,
    user_id uuid NOT NULL REFERENCES users(id),
    PRIMARY KEY(confirmation_token),
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz
);