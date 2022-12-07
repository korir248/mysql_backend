-- Your SQL goes here
create table users(user_id int not null auto_increment, user_name varchar(50) not null, email varchar(50) unique, primary key(user_id));