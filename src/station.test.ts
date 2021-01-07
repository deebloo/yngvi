import { EventEmitter } from 'events';
import { HID } from 'node-hid';
import test from 'ava';

import { Station } from './station';

class MockStation extends EventEmitter implements HID {
  private index = 0;

  constructor(private featureReport: number[][]) {
    super();
  }

  close() {}
  pause() {}
  resume() {}
  read(_callback: (err: any, data: number[]) => void) {}

  readSync() {
    return [];
  }

  readTimeout(_time_out: number) {
    return [];
  }

  sendFeatureReport(_data: number[] | Buffer) {
    return 0;
  }

  getFeatureReport(_report_id: number, _report_length: number) {
    const currentReport = this.featureReport[this.index];

    this.index++;

    return currentReport;
  }

  write(_values: number[] | Buffer) {
    return 0;
  }

  setNonBlocking(_no_block: boolean) {}
}

test.cb('it should create a device', (t) => {
  const station = new Station(
    new MockStation([
      [1, 197, 26, 113, 0, 56, 0, 108, 3, 255],
      [1, 197, 26, 120, 0, 102, 15, 65, 3, 255],
    ]),
    { pollInterval: 10 }
  );

  station.subscribe((res) => {
    t.deepEqual(res, {
      windSpeed: 4,
      rain: 1.08,
      outTemp: 38,
      outHumid: 65,
      windChill: 35,
    });

    t.end();
  });
});
