import { watch } from 'chokidar';
import { Observable } from 'rxjs';
import { readdirSync } from 'fs';
import { join } from 'path';
import csv from 'csvtojson';

const BASE_PATH = 'C:/Users/Danny/Documents/AcuRite Weather Station';

interface WeatherData {
  Timestamp: string;
  OutdoorTemperature: number;
  OutdoorHumidity: number;
  DewPoint: number;
  HeatIndex: number;
  WindChill: number;
  BarometricPressure: number;
  Rain: number;
  WindSpeed: number;
  WindAverage: number;
  PeakWind: number;
  WindDirection: number;
  IndoorTemperature: number;
  IndoorHumidity: number;
}

const source = new Observable((subscriber) => {
  const watcher = watch(BASE_PATH, {
    ignored: /(^|[\/\\])\../, // ignore dotfiles
    persistent: true,
  });

  watcher.on('add', async (path) => {
    const res = await parseToJson(path);

    subscriber.next(res);
  });
});

source.subscribe((res) => {
  console.log('#######################');

  console.log(res);
});

function parseToJson(path: string): PromiseLike<WeatherData[]> {
  return csv({
    colParser: {
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
  })
    .fromFile(path)
    .then((res) => {
      return res.map((entry) => {
        const final: Record<string, any> = {};

        Object.keys(entry).forEach((key) => {
          final[key.split(' ').join('')] = entry[key];
        });

        return final as WeatherData;
      });
    });
}
