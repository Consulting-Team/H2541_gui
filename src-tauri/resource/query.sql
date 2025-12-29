SELECT
device_created_time,
name,
bool_v,
double_v
FROM t2x_dsme_devicedata.device_telemetries
WHERE entity_id = ?
AND second_id = ?
AND name = ?
$CONDITION;