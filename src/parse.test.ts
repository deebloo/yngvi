import test from 'ava';
import { join } from 'path';
import { WeatherData } from './data';

import { csvToWeatherData, weatherDataToPoints } from './parse';

test('parseToJson() returns correct js object', async (t) => {
  const csvPath = join(__dirname, '../test-data/test-weather.csv');
  const weatherData = await csvToWeatherData(csvPath);

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

test('weatherDataToPoints() returns a list of Influx points', (t) => {
  const seed: WeatherData = {
    Timestamp: new Date(),
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
  };

  const res = weatherDataToPoints([
    { ...seed, Rain: 1 },
    { ...seed, Rain: 2 },
  ]);

  const { Timestamp, ...fields } = seed;

  t.deepEqual(res, [
    {
      measurement: 'weather',
      timestamp: seed.Timestamp,
      fields: { ...fields, Rain: 1 },
    },
    {
      measurement: 'weather',
      timestamp: seed.Timestamp,
      fields: { ...fields, Rain: 2 },
    },
  ]);
});
