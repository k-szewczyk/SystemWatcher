-- Your SQL goes here
CREATE TABLE stats (
   id SERIAL PRIMARY KEY,
   node SERIAL NOT NULL,
   stat_type INT NOT NULL,
   value INT NOT NULL,
   CONSTRAINT fk_stat_type
        FOREIGN KEY(stat_type)
            REFERENCES stat_types(id)
            ON DELETE CASCADE,
   CONSTRAINT fk_node
        FOREIGN KEY(node)
            REFERENCES nodes(id)
            ON DELETE CASCADE
);
