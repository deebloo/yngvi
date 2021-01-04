// import { config } from 'dotenv';
// import { concatMap, map } from 'rxjs/operators';

import * as usb from 'usb';

// const device = new HID(
//   '\\\\?\\hid#vid_24c0&pid_0003#6&362ca017&0&0000#{4d1e55b2-f16f-11cf-88cb-001111000030}'
// );

// USB constants for HID
const USB_HID_GET_REPORT = 0x01;
const USB_HID_SET_REPORT = 0x09;
const USB_HID_INPUT_REPORT = 0x0100;
const USB_HID_OUTPUT_REPORT = 0x0200;

const device = usb.findByIds(0x24c0, 0x003);

device.open();

readReport1().then((res) => {
  console.log(res);
});

function readReport1() {
  return readReport(1, 10);
}

function readReport2() {
  return readReport(2, 25);
}

function readReport3() {
  return readReport(3, 33);
}

function readReport(reportNumber: number, length: number) {
  return new Promise<Buffer | undefined>((resolve) => {
    device.controlTransfer(
      usb.LIBUSB_RECIPIENT_INTERFACE +
        usb.LIBUSB_REQUEST_TYPE_CLASS +
        usb.LIBUSB_ENDPOINT_IN,
      USB_HID_GET_REPORT,
      USB_HID_INPUT_REPORT + reportNumber,
      0x0,
      length,
      (_, res) => {
        resolve(res);
      }
    );
  });
}
