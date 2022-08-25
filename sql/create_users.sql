DROP TABLE users;

CREATE TABLE users(
    uid  serial PRIMARY KEY ,
    email varchar(128),
    nick_name varchar(16),
    student_id varchar(16),
    contact varchar(64),
    passwd varchar(128)
);


INSERT INTO users (
    email,
    nick_name,
    student_id,
    contact,
    passwd
  )
VALUES (
    'faa',
    'fadf',
    'dafasf',
    'sdafdd',
    'sd121'
  );