/* MIGRATION TEMPLATE */
/* DO NOT DELETE TEMPLATE COMMENTS! */

/* --- */

/*START MIGRATION UP*/

CREATE TABLE my_table (
    id INT PRIMARY KEY AUTO_INCREMENT
);

/*END MIGRATION UP*/

/* --- */

/*START MIGRATION DOWN*/

DROP TABLE my_table;

/*END MIGRATION DOWN*/
