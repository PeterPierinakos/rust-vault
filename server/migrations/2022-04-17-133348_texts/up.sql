CREATE TABLE texts (
	id SERIAL PRIMARY KEY,
	owner NUMERIC NOT NULL,
	content VARCHAR (1000) NOT NULL
);
