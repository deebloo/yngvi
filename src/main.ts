import { deepEqual } from 'assert';
import { HID } from 'node-hid';

type ReportKey = [number, number];

interface Report {
  type: 1 | 8;
}

interface RainReport {}

const VENDOR_ID = 0x24c0;
const PRODUCT_ID = 0x0003;
const REPORT_1: ReportKey = [0x0100 + 1, 10];
const REPORT_2: ReportKey = [0x0100 + 2, 25];

const R1_INTERVAL = 18 * 1000;

const device = new HID(VENDOR_ID, PRODUCT_ID);
let r1NextRead = 0;

pollDevice();

function pollDevice() {
  if (Date.now() >= r1NextRead) {
    const report = device.getFeatureReport(...REPORT_1);
    const flavor = decodeMessageFlavor(report);

    let data: any = {
      windSpeed: round(decodeWindSpeed(report), 0),
    };

    switch (flavor) {
      case 1: {
        data.rain = round(decodeRain(report), 2);

        break;
      }

      case 8: {
        data.outTemp = round(decodeOutTemp(report), 0);
        data.outHumid = decodeOutHumid(report);

        break;
      }
    }

    r1NextRead = Date.now() + R1_INTERVAL;

    console.log('Message Flavor:', flavor);
    console.table(data);
  }

  const delay = r1NextRead - Date.now() + 1;

  console.log(`next read in ${delay / 1000} seconds`);

  setTimeout(pollDevice, delay);
}

function decodeRain(data: number[]) {
  const cm = (((data[6] & 0x3f) << 7) | (data[7] & 0x7f)) * 0.0254;

  return cm / 2.54;
}

function decodeOutTemp(data: number[]) {
  const a = (data[5] & 0x0f) << 7;
  const b = data[6] & 0x7f;
  const celcius = (a | b) / 18.0 - 40.0;

  return (celcius * 9) / 5 + 32;
}

function decodeOutHumid(data: number[]) {
  return data[7];
}

function decodeMessageFlavor(data: number[]) {
  return data[3] & 0x0f;
}

function decodeWindSpeed(data: number[]) {
  const n = ((data[4] & 0x1f) << 3) | ((data[5] & 0x70) >> 4);

  if (n == 0) {
    return 0.0;
  }

  return (0.8278 * n + 1.0) / 1.609;
}

function round(value: number, decimals: number) {
  return Number(Math.round(Number(value + 'e' + decimals)) + 'e-' + decimals);
}
