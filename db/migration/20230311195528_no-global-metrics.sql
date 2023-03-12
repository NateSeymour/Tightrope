/* MIGRATION TEMPLATE */
/* DO NOT DELETE TEMPLATE COMMENTS! */

/* --- */

/*START MIGRATION UP*/

CREATE TABLE metric_old AS SELECT * FROM metric;
DELETE FROM metric WHERE global = true;
ALTER TABLE metric DROP COLUMN global;

/*END MIGRATION UP*/

/* --- */

/*START MIGRATION DOWN*/

ALTER TABLE metric ADD COLUMN global BOOLEAN NOT NULL;

/*END MIGRATION DOWN*/