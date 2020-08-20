create table if not exists users (
    id int(11) not null auto_increment,
    username varchar(255) not null unique,
    password varchar(255) not null,
    name varchar(255),
    surname varchar(255),
    primary key (id)
)
