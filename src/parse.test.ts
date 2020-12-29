import test from 'ava';
import { join } from 'path';

import { parseToJson } from './parse';

test('parseToJson() returns correct js object', async (t) => {
  const csvPath = join(__dirname, '../test-data/test-weather.csv');
  const weatherData = await parseToJson(csvPath);

  t.deepEqual(weatherData, [
    {
      Timestamp: weatherData[0]?.Timestamp,
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
