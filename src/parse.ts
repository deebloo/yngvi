import csv from 'csvtojson';
import * as Influx from 'influx';

import { WeatherDataOg, WeatherData } from './data';

export function csvToWeatherData(path: string): PromiseLike<WeatherData[]> {
  return createCSVParser()
    .fromFile(path)
    .then((res) => res.map(normalizeWeatherData));
}

export function weatherDataToPoints(data: WeatherData[]): Influx.IPoint[] {
  return data.map(({ Timestamp: timestamp, ...fields }) => ({
    measurement: 'weather',
    fields,
    timestamp,
  }));
}

function createCSVParser() {
  return csv({
    colParser: {
      Timestamp: (val) => new Date(val),
      'Outdoor Temperature': Number,
      'Outdoor Humidity': Number,
      'Dew Point': Number,
      'Heat Index': Number,
      'Wind Chill': Number,
      'Barometric Pressure': Number,
      Rain: Number,
      'Wind Speed': Number,
      'Wind Average': Number,
      'Peak Wind': Number,
      'Wind Direction': Number,
      'Indoor Temperature': Number,
      'Indoor Humidity': Number,
    },
  });
}

function normalizeWeatherData(data: WeatherDataOg): WeatherData {
  const final: Record<string, any> = {};

  for (let key in data) {
    const newKey = key.split(' ').join('');

    final[newKey] = data[key as keyof WeatherDataOg];
  }

  return final as WeatherData;
}
