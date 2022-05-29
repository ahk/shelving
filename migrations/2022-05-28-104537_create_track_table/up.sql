-- up.sql
CREATE TABLE track (
    -- A very longwinded explanation of why AUTOINCREMENT is excluded from id
    --   https://sqlite.org/autoinc.html
    -- and why INTEGER must be the exact type
    --   https://sqlite.org/lang_createtable.html#:~:text=5.%20ROWIDs%20and%20the%20INTEGER%20PRIMARY%20KEY
    --
    -- Short of it is that this specific signature removes some overhead compared to AUTOINCREMENT (at the expense of correctness
    -- with maybe the SQL spec) and adds robustness if reaching the max row id. Omitting it means we can reuse rows ids of deleted rows.
    -- My thinking is you'll never hit that limit naturally. One maybe gotcha is failing a constraint check can create empty rowid, and therefore skipped id values.
    -- You can imagine badly performing code (and maybe contending row creation?) eating up all the ids ... but even if you spent all day 
    -- generating constraint violations (e.g. uniqueness constraint check failures), you should never hit the limit of a 64bit counter.
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);