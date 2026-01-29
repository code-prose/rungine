CREATE TABLE tf_idf (
    word varchar not null,
    document varchar not null,
    tf_idf float not null,
    primary key (word),
    foreign key (document) references document(name)
)
