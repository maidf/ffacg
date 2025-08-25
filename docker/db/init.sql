DROP TABLE if EXISTS t_user_love;

DROP TABLE if EXISTS t_user_black;

DROP TABLE if EXISTS t_medal;

DROP TABLE if EXISTS t_profile;

DROP TABLE if EXISTS t_wallet;

DROP TABLE if EXISTS t_wallet_record;

DROP TABLE if EXISTS t_security;

DROP TABLE if EXISTS t_user_dyn;

DROP TABLE if EXISTS t_anime_history;

DROP TABLE if EXISTS t_anime_mask;

DROP TABLE if EXISTS t_fansub;

DROP TABLE if EXISTS t_publisher;

DROP TABLE if EXISTS t_anime;

DROP TABLE IF EXISTS t_user;

DROP TYPE if EXISTS Currency;

CREATE TABLE t_user (
    id VARCHAR(36) PRIMARY KEY,
    account VARCHAR(36) NOT NULL UNIQUE,
    name VARCHAR(36) NOT NULL DEFAULT '新用户',
    pass VARCHAR(44) NOT NULL,
    sex VARCHAR(5) DEFAULT '保密',
    birth BIGINT,
    signature TEXT,
    level INTEGER DEFAULT 0,
    exp INTEGER DEFAULT 0,
    status BOOLEAN DEFAULT true
);

CREATE TABLE t_profile (
    id VARCHAR(36) PRIMARY KEY,
    url TEXT,
    uid VARCHAR(36),
    constraint fk_profile_uid Foreign Key (uid) REFERENCES t_user (id) on delete CASCADE on update CASCADE
);

CREATE TABLE t_security (
    id VARCHAR(36) PRIMARY KEY,
    email VARCHAR(55),
    phone VARCHAR(55),
    idc VARCHAR(55),
    uid VARCHAR(36),
    constraint fk_security_uid Foreign Key (uid) REFERENCES t_user (id) on delete CASCADE on update CASCADE
);

CREATE TABLE t_medal (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(10) NOT NULL UNIQUE,
    level INTEGER DEFAULT 0,
    exp INTEGER DEFAULT 0,
    uid VARCHAR(36),
    constraint fk_medal_uid Foreign Key (uid) REFERENCES t_user (id) on delete CASCADE on update CASCADE
);

CREATE TABLE t_wallet (
    id VARCHAR(36) PRIMARY KEY,
    coin DECIMAL NOT NULL DEFAULT 0,
    gold DECIMAL NOT NULL DEFAULT 0,
    uid VARCHAR(36),
    constraint fk_wallet_uid Foreign Key (uid) REFERENCES t_user (id) on delete CASCADE on update CASCADE
);

CREATE TYPE Currency AS ENUM ('Coin', 'Gold');

CREATE TABLE t_wallet_record (
    id VARCHAR(36) PRIMARY KEY,
    type Currency NOT NULL DEFAULT 'Coin',
    time BIGINT NOT NULL,
    num DECIMAL NOT NULL DEFAULT 0,
    uid VARCHAR(36),
    constraint fk_wallet_record_uid Foreign Key (uid) REFERENCES t_user (id) on delete CASCADE on update CASCADE
);

CREATE TABLE t_anime (
    id VARCHAR(36) PRIMARY KEY,
    title VARCHAR(50) NOT NULL,
    size INTEGER,
    magnet TEXT,
    tracker TEXT
);

CREATE TABLE t_fansub (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(50),
    avatar TEXT,
    aid VARCHAR(36),
    constraint fk_fansub_aid Foreign Key (aid) REFERENCES t_anime (id) on delete CASCADE on update CASCADE
);

CREATE TABLE t_publisher (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(50),
    avatar TEXT,
    aid VARCHAR(36),
    constraint fk_publisher_aid Foreign Key (aid) REFERENCES t_anime (id) on delete CASCADE on update CASCADE
);

CREATE TABLE t_user_dyn (
    id VARCHAR(36) PRIMARY KEY,
    title VARCHAR(50) NOT NULL,
    time BIGINT NOT NULL,
    msg TEXT,
    aid VARCHAR(36),
    uid VARCHAR(36),
    constraint fk_user_dyn_aid Foreign Key (aid) REFERENCES t_anime (id) on delete CASCADE on update CASCADE,
    constraint fk_user_dyn_uid Foreign Key (uid) REFERENCES t_user (id) on delete CASCADE on update CASCADE
);

CREATE TABLE t_user_love (
    id VARCHAR(36) PRIMARY KEY,
    uid VARCHAR(36),
    love VARCHAR(36),
    constraint fk_user_dyn_uid Foreign Key (uid) REFERENCES t_user (id) on delete CASCADE on update CASCADE,
    constraint fk_user_dyn_love Foreign Key (uid) REFERENCES t_user (id) on delete CASCADE on update CASCADE
);

CREATE TABLE t_user_black (
    id VARCHAR(36) PRIMARY KEY,
    uid VARCHAR(36),
    black VARCHAR(36),
    constraint fk_user_dyn_uid Foreign Key (uid) REFERENCES t_user (id) on delete CASCADE on update CASCADE,
    constraint fk_user_dyn_black Foreign Key (uid) REFERENCES t_user (id) on delete CASCADE on update CASCADE
);

CREATE TABLE t_anime_history (
    id VARCHAR(36) PRIMARY KEY,
    time BIGINT NOT NULL,
    aid VARCHAR(36),
    uid VARCHAR(36),
    constraint fk_user_dyn_aid Foreign Key (aid) REFERENCES t_anime (id) on delete CASCADE on update CASCADE,
    constraint fk_user_dyn_uid Foreign Key (uid) REFERENCES t_user (id) on delete CASCADE on update CASCADE
);

CREATE TABLE t_anime_mask (
    id VARCHAR(36) PRIMARY KEY,
    group_name VARCHAR(10) NOT NULL DEFAULT '默认分组',
    aid VARCHAR(36),
    uid VARCHAR(36),
    constraint fk_user_dyn_aid Foreign Key (aid) REFERENCES t_anime (id) on delete CASCADE on update CASCADE,
    constraint fk_user_dyn_uid Foreign Key (uid) REFERENCES t_user (id) on delete CASCADE on update CASCADE
);