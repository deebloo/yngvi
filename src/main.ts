import { IPoint } from 'influx';
import { HID } from 'node-hid';
import { concatMap } from 'rxjs/operators';

import { createDbConnection } from './db';
import { Station, WeatherRecord } from './station';

const { INFLUXDB_HOST = 'localhost' } = process.env;
const VENDOR_ID = 0x24c0;
const PRODUCT_ID = 0x0003;

console.log('App Starting...');

const influx = createDbConnection({ host: INFLUXDB_HOST });
const hid = new HID(VENDOR_ID, PRODUCT_ID);
const station = new Station(hid);

station.pipe(concatMap(writePoint)).subscribe((point) => {
  console.log(point.timestamp);
  console.table([point.fields]);
});

function writePoint(fields: WeatherRecord) {
  const point: IPoint = {
    measurement: 'weather',
    timestamp: new Date(),
    fields,
  };

  return influx.writePoints([point]).then(() => point);
}
