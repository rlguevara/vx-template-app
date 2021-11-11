CREATE USER odoo WITH PASSWORD 'odoo_pg_pass' CREATEDB;
CREATE DATABASE odoo
    WITH 
    OWNER = odoo
    ENCODING = 'UTF8'
    LC_COLLATE = 'en_US.utf8'
    LC_CTYPE = 'en_US.utf8'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1;

CREATE USER keycloak WITH PASSWORD 'password' CREATEDB;
CREATE DATABASE keycloak
    WITH 
    OWNER = root
    ENCODING = 'UTF8'
    LC_COLLATE = 'en_US.utf8'
    LC_CTYPE = 'en_US.utf8'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1;

CREATE USER hasura WITH PASSWORD 'hasura' CREATEDB;
CREATE DATABASE hasura
    WITH 
    OWNER = hasura
    ENCODING = 'UTF8'
    LC_COLLATE = 'en_US.utf8'
    LC_CTYPE = 'en_US.utf8'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1;
