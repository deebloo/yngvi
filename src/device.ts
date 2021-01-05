import { EventEmitter } from 'events';
import { HID } from 'node-hid';

type ReportKey = [number, number];

const REPORT_1: ReportKey = [0x0100 + 1, 10];
// const REPORT_2: ReportKey = [0x0100 + 2, 25];

export class Device extends EventEmitter {
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
        windSpeed: this.round(this.decodeWindSpeed(report), 0),
      };

      if (flavor === 1) {
        fields.rain = this.round(this.decodeRain(report), 2);
      } else {
        fields.outTemp = this.round(this.decodeOutTemp(report), 0);
        fields.outHumid = this.decodeOutHumid(report);
        fields.windChill = this.calculateWindChill(fields.outTemp, fields.windSpeed);
      }

      this.r1NextRead = Date.now() + 18 * 1000;

      this.emit('data', { measurement: 'weather', timestamp, fields });
    }

    setTimeout(() => this.startListening(), 18 * 1000);
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

  calculateWindChill(temp: number, windsSpeed: number): number {
    if (windsSpeed < 3) {
      return temp;
    }

    return Math.round(
      35.74 + 0.6215 * temp - 35.75 * windsSpeed ** 0.16 + 0.4275 * temp * windsSpeed ** 0.16
    );
  }

  round(value: number, decimals: number) {
    return Number(Math.round(Number(value + 'e' + decimals)) + 'e-' + decimals);
  }
}
