import * as Influx from 'influx';
import { config } from 'dotenv';
import { concatMap, map } from 'rxjs/operators';

config();

import { createDbConnection } from './db';
import { watchData } from './watcher';
import { WeatherData } from './data';

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
  useHistoricalData: Boolean(USE_HISTORICAL_DATA),
}).pipe(
  map<WeatherData[], Influx.IPoint[]>((res) => {
    return res.map(({ Timestamp: timestamp, ...fields }) => ({
      measurement: 'weather',
      fields,
      timestamp,
    }));
  }),
  concatMap((writePoints) => influx.writePoints(writePoints))
);

dbUpdates.subscribe(() => {
  console.log(`###################################`);
  console.log(`Data added on ${new Date().toString()}`);
  console.log(`###################################`);
});

console.log(`Watching data in ${ACURITE_DATA}...`);
