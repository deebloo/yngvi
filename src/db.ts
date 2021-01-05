import * as Influx from 'influx';

export function createDbConnection(options: Influx.ISingleHostConfig) {
  return new Influx.InfluxDB({
    database: 'weather',
    schema: [
      {
        measurement: 'weather',
        fields: {
          outTemp: Influx.FieldType.FLOAT,
          outHumid: Influx.FieldType.FLOAT,
          rain: Influx.FieldType.FLOAT,
          windSpeed: Influx.FieldType.FLOAT,
        },
        tags: [],
      },
    ],
    ...options,
  });
}
