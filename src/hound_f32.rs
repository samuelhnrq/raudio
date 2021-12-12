use std::{fs, io, path};

type HoundWriter = hound::WavWriter<io::BufWriter<fs::File>>;
type BufferedFile = io::BufReader<fs::File>;
type WavFileReader = hound::WavReader<BufferedFile>;

pub struct F32HoundWriter {
  writer: HoundWriter,
  spec: hound::WavSpec,
}

impl F32HoundWriter {
  pub fn create<P: AsRef<path::Path>>(
    writer: P,
    spec: hound::WavSpec,
  ) -> Result<Self, hound::Error> {
    hound::WavWriter::create(writer, spec).map(|writer| F32HoundWriter {
      spec: spec,
      writer: writer,
    })
  }

  pub fn write_sample(&mut self, sample: f32) -> Result<(), hound::Error> {
    if self.spec.sample_format == hound::SampleFormat::Float {
      return self.writer.write_sample(sample);
    }
    let size = self.spec.bits_per_sample;
    if size <= 16 {
      return self
        .writer
        .write_sample((sample * i16::MAX as f32).round() as i16);
    }
    return self
      .writer
      .write_sample((sample * i32::MAX as f32).round() as i32);
  }
}

pub fn ensure_f32_samples(reader: WavFileReader) -> Box<dyn Iterator<Item = f32>> {
  let spec = reader.spec();
  if spec.sample_format == hound::SampleFormat::Float {
    println!("32 bit float wav samples");
    let samples = reader.into_samples::<f32>().map(|x| x.unwrap());
    return Box::new(samples);
  }
  let size = spec.bits_per_sample;
  if size <= 16 {
    println!("16 bit int wav samples");
    let samples = reader
      .into_samples::<i16>()
      .map(|x| x.unwrap())
      .map(|x| -> f32 { x as f32 / i16::MAX as f32 });
    return Box::new(samples);
  }
  println!("{} bit int int samples", spec.bits_per_sample);
  let samples = reader
    .into_samples::<i32>()
    .map(|x| x.unwrap())
    .map(|x| -> f32 { x as f32 / i32::MAX as f32 });
  return Box::new(samples);
}
