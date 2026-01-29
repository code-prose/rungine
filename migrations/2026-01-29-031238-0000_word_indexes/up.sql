CREATE TABLE word_indexes (
    word varchar not null
    , doc varchar not null
    , tf_idf float not null
    , primary key (word)
    , foreign key (doc) references document(name)
);
