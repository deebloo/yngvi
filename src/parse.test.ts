import test from 'ava';
import { join } from 'path';

import { parseToJson } from './parse';

test('parseToJson() returns correct js object', async (t) => {
  const csvPath = join(__dirname, '../test-data/test-weather.csv');

  t.deepEqual(await parseToJson(csvPath), [
    {
      Timestamp: new Date('2020-12-29T19:00:00.000Z'),
      OutdoorTemperature: 39.4,
      OutdoorHumidity: 35,
      DewPoint: 13,
      HeatIndex: 39,
      WindChill: 39,
      BarometricPressure: 30.12058,
      Rain: 0,
      WindSpeed: 2.485485,
      WindAverage: 1.864114,
      PeakWind: 6.213712,
      WindDirection: 22.5,
      IndoorTemperature: 74.4,
      IndoorHumidity: 26,
    },
  ]);
});
