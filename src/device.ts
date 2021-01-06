import { HID } from 'node-hid';
import { Subject } from 'rxjs';

import { calculateWindChill, round } from './util';

type ReportKey = [number, number];

const REPORT_1: ReportKey = [0x0100 + 1, 10];
// const REPORT_2: ReportKey = [0x0100 + 2, 25];

export interface WeatherRecord {
  windSpeed: number;
  rain: number;
  outTemp: number;
  outHumid: number;
  windChill: number;
}

export class Device extends Subject<WeatherRecord> {
  private r1NextRead = 0;
  private device = new HID(this.vendorId, this.productId);
  private latestRecordR1: Partial<WeatherRecord> = {};
  private pollInterval = 18 * 1000;

  constructor(private vendorId: number, private productId: number) {
    super();

    this.startListening();
  }

  private startListening(): NodeJS.Timer | undefined {
    if (Date.now() >= this.r1NextRead) {
      let report: number[];

      try {
        // attempt to read the R1 Report
        report = this.device.getFeatureReport(...REPORT_1);
      } catch {
        // IF there is an error getting the report from the device wait for regular interval and try again
        return setTimeout(() => this.startListening(), this.pollInterval);
      }

      const flavor = this.decodeMessageFlavor(report);

      // We always get the windSpeed back
      this.latestRecordR1.windSpeed = round(this.decodeWindSpeed(report), 0);

      // Only update the properties that we get for a given report
      if (flavor === 1) {
        this.latestRecordR1.rain = round(this.decodeRain(report), 2);
      } else {
        this.latestRecordR1.outTemp = round(this.decodeOutTemp(report), 0);
        this.latestRecordR1.outHumid = this.decodeOutHumid(report);
        this.latestRecordR1.windChill = calculateWindChill(
          this.latestRecordR1.outTemp,
          this.latestRecordR1.windSpeed
        );
      }

      this.r1NextRead = Date.now() + this.pollInterval;

      // Only write record to the databse if we have a value for each field
      if (Object.keys(this.latestRecordR1).length === 5) {
        super.next(this.latestRecordR1 as WeatherRecord);
      }
    }

    return setTimeout(() => this.startListening(), this.pollInterval);
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
