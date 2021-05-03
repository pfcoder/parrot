-- Your SQL goes here
CREATE TABLE form_trails (
	id INTEGER AUTO_INCREMENT PRIMARY KEY,
  hospital_name VARCHAR(128) NOT NULL,
  applicant VARCHAR(128) NOT NULL,
	city VARCHAR(128) NOT NULL,
	user VARCHAR(128) NOT NULL,
	contact TEXT NOT NULL,
	prod TEXT NOT NULL,
	model TEXT NOT NULL,
	month VARCHAR(128) NOT NULL,
	day VARCHAR(128) NOT NULL,
	bmonth VARCHAR(128) NOT NULL,
	bday VARCHAR(128) NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE form_buys (
	id INTEGER AUTO_INCREMENT PRIMARY KEY,
  per_com VARCHAR(128) NOT NULL,
  company TEXT NOT NULL,
	city VARCHAR(128) NOT NULL,
	name VARCHAR(128) NOT NULL,
	contact TEXT NOT NULL,
	prod TEXT NOT NULL,
	model TEXT NOT NULL,
	amount VARCHAR(128) NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE form_repairs (
	id INTEGER AUTO_INCREMENT PRIMARY KEY,
	hospital_name VARCHAR(128) NOT NULL,
	city VARCHAR(128) NOT NULL,
	name VARCHAR(128) NOT NULL,
	contact TEXT NOT NULL,
	prod TEXT NOT NULL,
	model TEXT NOT NULL,
	serial TEXT,
	year VARCHAR(128),
	month VARCHAR(128),
	question TEXT,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE form_register_pss (
	id INTEGER AUTO_INCREMENT PRIMARY KEY,
  username VARCHAR(128) NOT NULL,
  password VARCHAR(128) NOT NULL,
	passwordq TEXT,
	passworda TEXT,
	usertype TEXT NOT NULL,
	name TEXT NOT NULL,
	year VARCHAR(128) NOT NULL,
	month VARCHAR(128) NOT NULL,
	company TEXT NOT NULL,
	title TEXT NOT NULL,
	expertise TEXT NOT NULL,
	cell VARCHAR(128) NOT NULL,
	identity VARCHAR(128) NOT NULL,
	recommander VARCHAR(128),
	recommander_name VARCHAR(128),
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE form_register_bus (
	id INTEGER AUTO_INCREMENT PRIMARY KEY,
  username VARCHAR(128) NOT NULL,
  password VARCHAR(128) NOT NULL,
	passwordq TEXT,
	passworda TEXT,
	usertype TEXT NOT NULL,
	name TEXT NOT NULL,
	address TEXT NOT NULL,
	postcode TEXT,
	personincharge TEXT,
	tel TEXT,
	cell TEXT,
	province VARCHAR(128) NOT NULL,
	city VARCHAR(128) NOT NULL,
	mainbusiness TEXT,
	sellingbrand TEXT,
	recommander VARCHAR(128),
	recommander_name VARCHAR(128),
	subcompany VARCHAR(128),
	subcompany_name VARCHAR(128),
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);