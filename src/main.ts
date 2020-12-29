import * as Influx from 'influx';
import { createDbConnection } from './db';

import { watchFiles } from './watcher';

console.log('App Starting...');

const {
  ACURITE_DATA = 'C:/Users/Danny/Documents/AcuRite Weather Station',
  USE_HISTORICAL_DATA = false,
  INFLUXDB_HOST = 'localhost',
} = process.env;

if (!ACURITE_DATA) {
  throw new Error('No path the acurite data provided to ACURITE_DATA');
}

const influx = createDbConnection({
  host: INFLUXDB_HOST,
});

const source = watchFiles({
  path: ACURITE_DATA,
  useHistoricalData: Boolean(USE_HISTORICAL_DATA),
});

source.subscribe(async (res) => {
  const writePoints: Influx.IPoint[] = res.map((data) => {
    const { Timestamp: timestamp, ...fields } = data;

    return {
      measurement: 'weather',
      fields,
      timestamp,
    };
  });

  await influx.writePoints(writePoints);

  console.log(`Data added ${new Date().toString()}`);
});

console.log(`Watching data in ${ACURITE_DATA}...`);
