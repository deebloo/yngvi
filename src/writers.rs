use yngvi::{
    core::{InMemWriter, NoopWriter, StdoutWriter, Writer},
    influxdb::{Influx2Writer, InfluxWriter},
};

pub enum AppWriter {
    InfluxDB(InfluxWriter),
    InfluxDB2(Influx2Writer),
    InMemory(InMemWriter),
    Stdout(StdoutWriter),
    Noop(NoopWriter),
}

impl Writer for AppWriter {
    async fn write(&mut self, weather_reading: &yngvi::core::WeatherReading) -> Result<(), ()> {
        match self {
            AppWriter::InfluxDB(writer) => writer.write(weather_reading).await,
            AppWriter::InfluxDB2(writer) => writer.write(weather_reading).await,
            AppWriter::InMemory(writer) => writer.write(weather_reading).await,
            AppWriter::Stdout(writer) => writer.write(weather_reading).await,
            AppWriter::Noop(writer) => writer.write(weather_reading).await,
        }
    }
}
