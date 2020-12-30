import { config } from 'dotenv';
import { concatMap, map } from 'rxjs/operators';

import { createDbConnection } from './db';
import { watchData } from './watcher';
import { weatherDataToPoints } from './parse';

config();

console.log('App Starting...');

const {
  ACURITE_DATA,
  USE_HISTORICAL_DATA = false,
  INFLUXDB_HOST = 'localhost',
} = process.env;

if (!ACURITE_DATA) {
  throw new Error('No path the acurite data provided to ACURITE_DATA');
}

const influx = createDbConnection({
  host: INFLUXDB_HOST,
});

const dbUpdates = watchData({
  path: ACURITE_DATA,
  useHistoricalData: !!USE_HISTORICAL_DATA,
}).pipe(
  map(weatherDataToPoints),
  concatMap((writePoints) => influx.writePoints(writePoints))
);

dbUpdates.subscribe(() => {
  console.log(`###################################`);
  console.log(`Data added on ${new Date().toString()}`);
  console.log(`###################################`);
});

console.log(`Watching data in ${ACURITE_DATA}...`);
