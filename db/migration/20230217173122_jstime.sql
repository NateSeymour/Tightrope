/* MIGRATION TEMPLATE */
/* DO NOT DELETE TEMPLATE COMMENTS! */

/* --- */

/*START MIGRATION UP*/

ALTER TABLE weight MODIFY COLUMN seconds BIGINT UNSIGNED NOT NULL;
ALTER TABLE weight RENAME COLUMN seconds TO millis;
UPDATE weight SET millis=millis*1000;

/*END MIGRATION UP*/

/* --- */

/*START MIGRATION DOWN*/

UPDATE weight SET millis=millis/1000;
ALTER TABLE weight MODIFY COLUMN millis INT UNSIGNED NOT NULL;
ALTER TABLE weight RENAME COLUMN millis TO seconds;

/*END MIGRATION DOWN*/