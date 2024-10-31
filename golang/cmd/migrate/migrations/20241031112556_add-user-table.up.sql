create table if not exists users (
    `id` int unsigned not null auto_increment,
    `first_name` varchar(255) not null,
    `last_name` varchar(255) not null,
    `email` varchar(255) not null,
    `password` varchar(255) not null,
    `create_at` datetime not null,

    primary key (`id`),
    unique key (`email`)
);