import * as Influx from 'influx';

export function createDbConnection(options: Influx.ISingleHostConfig) {
  return new Influx.InfluxDB({
    database: 'weather',
    schema: [
      {
        measurement: 'weather',
        fields: {
          OutdoorTemperature: Influx.FieldType.FLOAT,
          OutdoorHumidity: Influx.FieldType.FLOAT,
          DewPoint: Influx.FieldType.FLOAT,
          HeatIndex: Influx.FieldType.FLOAT,
          WindChill: Influx.FieldType.FLOAT,
          BarometricPressure: Influx.FieldType.FLOAT,
          Rain: Influx.FieldType.FLOAT,
          WindSpeed: Influx.FieldType.FLOAT,
          WindAverage: Influx.FieldType.FLOAT,
          PeakWind: Influx.FieldType.FLOAT,
          WindDirection: Influx.FieldType.FLOAT,
          IndoorTemperature: Influx.FieldType.FLOAT,
          IndoorHumidity: Influx.FieldType.FLOAT,
        },
        tags: [],
      },
    ],
    ...options,
  });
}
