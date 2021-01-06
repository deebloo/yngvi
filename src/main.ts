import { IPoint } from 'influx';
import { createDbConnection } from './db';
import { Device } from './device';

const VENDOR_ID = 0x24c0;
const PRODUCT_ID = 0x0003;

console.log('App Starting...');

const { INFLUXDB_HOST = 'localhost' } = process.env;

const influx = createDbConnection({
  host: INFLUXDB_HOST,
});

const device = new Device(VENDOR_ID, PRODUCT_ID);

device.subscribe(async (fields) => {
  const point: IPoint = {
    measurement: 'weather',
    timestamp: new Date(),
    fields,
  };

  await influx.writePoints([point]);

  console.log(point.timestamp);
  console.table([point.fields]);
});
