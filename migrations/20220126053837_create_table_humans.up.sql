DROP TABLE IF EXISTS humans;

CREATE TABLE humans (
    human_id INT GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL,
    PRIMARY KEY (human_id)
);

ALTER TABLE cats
    ADD COLUMN human_id INT;

ALTER TABLE cats
    ADD FOREIGN KEY (human_id) REFERENCES humans;
