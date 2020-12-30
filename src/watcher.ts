import { Observable } from 'rxjs';
import { watch } from 'chokidar';

import { csvToWeatherData } from './parse';
import { WeatherData } from './data';

export interface WatchOptions {
  path: string;
  useHistoricalData: boolean;
}

export function watchData({ path, useHistoricalData }: WatchOptions) {
  return new Observable<WeatherData[]>((subscriber) => {
    const watcher = watch(path, {
      ignored: /(^|[\/\\])\../, // ignore dotfiles
      persistent: true,
      ignoreInitial: !useHistoricalData,
      usePolling: true, // Issue with watching docker volumes
    });

    watcher.on('add', async (path) => {
      const res = await csvToWeatherData(path);

      subscriber.next(res);
    });
  });
}
