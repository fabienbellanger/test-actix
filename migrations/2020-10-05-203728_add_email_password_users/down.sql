ALTER TABLE `users` DROP INDEX unique_email;

ALTER TABLE `users` DROP `email`;
ALTER TABLE `users` DROP `password`;
