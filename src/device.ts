import { HID } from 'node-hid';
import { Subject } from 'rxjs';

import { calculateWindChill, round } from './util';

type ReportKey = [number, number];

const REPORT_1: ReportKey = [0x0100 + 1, 10];
// const REPORT_2: ReportKey = [0x0100 + 2, 25];

export class Device extends Subject<any> {
  private r1NextRead = 0;
  private device = new HID(this.vendorId, this.productId);

  constructor(private vendorId: number, private productId: number) {
    super();

    this.startListening();
  }

  private startListening() {
    if (Date.now() >= this.r1NextRead) {
      const report = this.device.getFeatureReport(...REPORT_1);
      const flavor = this.decodeMessageFlavor(report);
      const timestamp = new Date();

      const fields: any = {
        windSpeed: round(this.decodeWindSpeed(report), 0),
      };

      if (flavor === 1) {
        fields.rain = round(this.decodeRain(report), 2);
      } else {
        fields.outTemp = round(this.decodeOutTemp(report), 0);
        fields.outHumid = this.decodeOutHumid(report);
        fields.windChill = calculateWindChill(fields.outTemp, fields.windSpeed);
      }

      this.r1NextRead = Date.now() + 18 * 1000;

      super.next({ measurement: 'weather', timestamp, fields });
    }

    setTimeout(() => this.startListening(), 18 * 1000);
  }

  next() {
    throw new Error('Cannot push values to Device');
  }

  decodeMessageFlavor(data: number[]) {
    return data[3] & 0x0f;
  }

  decodeWindSpeed(data: number[]) {
    const n = ((data[4] & 0x1f) << 3) | ((data[5] & 0x70) >> 4);

    if (n == 0) {
      return 0.0;
    }

    return (0.8278 * n + 1.0) / 1.609;
  }

  decodeRain(data: number[]) {
    const cm = (((data[6] & 0x3f) << 7) | (data[7] & 0x7f)) * 0.0254;

    return cm / 2.54;
  }

  decodeOutTemp(data: number[]) {
    const a = (data[5] & 0x0f) << 7;
    const b = data[6] & 0x7f;
    const celcius = (a | b) / 18.0 - 40.0;

    return (celcius * 9) / 5 + 32;
  }

  decodeOutHumid(data: number[]) {
    return data[7];
  }
}
