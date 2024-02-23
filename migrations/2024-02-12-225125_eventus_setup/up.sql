create table eventus.user_ (
    id serial primary key,
    username varchar(255) not null unique,
    password_ varchar(255) not null unique,
    active boolean not null
);

create table eventus.authority (
    id serial primary key,
    name_ varchar(255) not null,
    user_id int not null,
    foreign key (user_id) references eventus.user_(id)
);

create table eventus.event_ (
    id serial primary key,
    name_ varchar(255) not null,
    description_ varchar(2048) not null,
    creator int not null,
    creation_date timestamp with time zone not null,
    start_ timestamp with time zone not null,
    end_ timestamp with time zone not null,
    seats int not null,
    approved boolean not null,
    foreign key (creator) references eventus.user_(id)
);

create table eventus.subscription (
    id serial primary key,
    user_id int not null,
    event_id int not null,
    creation_date timestamp with time zone not null,
    rating int,
    comment varchar(2048),
    foreign key (user_id) references eventus.user_(id),
    foreign key (event_id) references eventus.event_(id),
    unique (user_id, event_id) 
);
