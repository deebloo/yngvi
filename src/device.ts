import { devices, HID } from 'node-hid';
import { Observable } from 'rxjs';

const VENDOR_ID = 9408;
const PRODUCT_ID = 3;

export function getStation() {
  const device = new HID(
    '\\\\?\\hid#vid_24c0&pid_0003#6&362ca017&0&0000#{4d1e55b2-f16f-11cf-88cb-001111000030}'
  );

  console.log(device.getFeatureReport(0, 100));

  return new Observable((subscriber) => {
    device.on('data', (res) => {
      subscriber.next(res);
    });

    device.on('error', (err) => {
      subscriber.error(err);
    });
  });
}
