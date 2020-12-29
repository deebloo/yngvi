export interface WeatherDataOg {
  Timestamp: string;
  'Outdoor Temperature': number;
  'Outdoor Humidity': number;
  'Dew Point': number;
  'Heat Index': number;
  'Wind Chill': number;
  'Barometric Pressure': number;
  Rain: number;
  'Wind Speed': number;
  'Wind Average': number;
  'Peak Wind': number;
  'Wind Direction': number;
  'Indoor Temperature': number;
  'Indoor Humidity': number;
}

export interface WeatherData {
  Timestamp: Date;
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
