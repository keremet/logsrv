drop table log;

create table log (
	id integer not null auto_increment primary key,
	dt datetime(6) not null default current_timestamp(6),
	log_level integer not null check(log_level in (1, 2, 3, 4)),
	txt text not null
);

