import { Observable } from 'rxjs';
import { watch } from 'chokidar';

import { parseToJson } from './parse';
import { WeatherData } from './data';

export interface WatchOptions {
  path: string;
  useHistoricalData: boolean;
}

export function watchFiles({ path, useHistoricalData }: WatchOptions) {
  return new Observable<WeatherData[]>((subscriber) => {
    const watcher = watch(path, {
      ignored: /(^|[\/\\])\../, // ignore dotfiles
      persistent: true,
      ignoreInitial: !useHistoricalData,
      usePolling: true,
    });

    watcher.on('add', async (path) => {
      const res = await parseToJson(path);

      subscriber.next(res);
    });
  });
}
