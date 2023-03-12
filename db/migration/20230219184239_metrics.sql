/* MIGRATION TEMPLATE */
/* DO NOT DELETE TEMPLATE COMMENTS! */

/* --- */

/*START MIGRATION UP*/

START TRANSACTION;

CREATE TABLE measurement (
    measurement_id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    metric_id INT NOT NULL,
    username VARCHAR(100) NOT NULL,
    millis BIGINT UNSIGNED NOT NULL,
    value DOUBLE NOT NULL
);

CREATE TABLE metric (
    metric_id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    metric_type ENUM('Mass', 'Speed', 'Time', 'Length', 'PsychScale', 'Binary') NOT NULL,
    username VARCHAR(100),
    global BOOLEAN NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT
);

CREATE TABLE goal (
    goal_id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    metric_id INT UNSIGNED NOT NULL,
    username VARCHAR(100) NOT NULL,
    target_value DOUBLE NOT NULL,
    start_millis BIGINT UNSIGNED NOT NULL,
    end_millis BIGINT UNSIGNED NOT NULL,
    name VARCHAR(100),
    description TEXT
);

CREATE TABLE event (
    event_id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(100) NOT NULL,
    millis BIGINT UNSIGNED NOT NULL,
    name VARCHAR(100),
    description TEXT
);

INSERT INTO metric(metric_id, name, metric_type, global)
    VALUES(1, 'Weight', 'Mass', true),
          (2, 'Perceived Energy', 'PsychScale', true),
          (3, 'Tummy Circumference', 'Length', true),
          (4, 'Upper Arm Circumference', 'Length', true),
          (5, 'Thigh Circumference', 'Length', true),
          (6, 'Calf Circumference', 'Length', true);

INSERT INTO measurement(metric_id, username, millis, value)
    SELECT 1, weight.username, weight.millis, weight.kilograms FROM weight;

DROP TABLE weight;

COMMIT;

/*END MIGRATION UP*/

/* --- */

/*START MIGRATION DOWN*/

START TRANSACTION;

CREATE TABLE weight (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(100) NOT NULL,
    kilograms DOUBLE NOT NULL,
    millis BIGINT UNSIGNED NOT NULL
);

INSERT INTO weight(username, kilograms, millis)
    SELECT measurement.username, measurement.value, measurement.millis
    FROM measurement WHERE metric_id = 1;

DROP TABLE measurement;
DROP TABLE metric;
DROP TABLE goal;
DROP TABLE event;

COMMIT;

/*END MIGRATION DOWN*/
