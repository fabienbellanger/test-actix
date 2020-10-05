ALTER TABLE `users` ADD `email` VARCHAR(255) NOT NULL;
ALTER TABLE `users` ADD `password` VARCHAR(128) NOT NULL;

ALTER TABLE `users` ADD CONSTRAINT unique_email UNIQUE (email);
