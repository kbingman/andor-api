CREATE TABLE people (
    id SERIAL PRIMARY KEY,
    name character varying(255) NOT NULL UNIQUE,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE episodes (
    id SERIAL PRIMARY KEY,
    title character varying(255) NOT NULL UNIQUE,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE people_episodes (
    episode_id int REFERENCES episodes (id) ON UPDATE CASCADE ON DELETE CASCADE,
    person_id int REFERENCES people (id) ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY(episode_id, person_id)
);
