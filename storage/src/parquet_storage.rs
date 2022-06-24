use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use arrow::array::{Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::{ArrowReader, ArrowWriter, ParquetFileArrowReader};
use parquet::column::writer::ColumnWriter;
use parquet::file::properties::{WriterProperties, WriterVersion};
use parquet::file::serialized_reader::SerializedFileReader;
use parquet::file::writer::{FileWriter, SerializedFileWriter};
use parquet::schema::parser::parse_message_type;

// use parquet::arrow::ParquetRecordBatchStreamBuilder;
// use parquet::arrow::ParquetRecordBatchStreamBuilder;
use crate::reader::Reader;
use crate::writer::Writer;

#[derive(Debug)]
pub struct ParquetStorage {
}

impl ParquetStorage {
    fn new() -> Self {
        let storage_obj = ParquetStorage {};
        storage_obj
    }
}

impl Writer for ParquetStorage {
    fn write(&self, key: String, value: String) {
        let path = Path::new("/Users/admin/Documents/code/aurach/data/sample.parquet");
        let fields = vec![Field::new("key", DataType::Utf8, false), Field::new("value", DataType::Utf8, false)];
        let schema = Arc::new(Schema::new(fields));
        let keys = Arc::new(StringArray::from(vec![key]));
        let values = Arc::new(StringArray::from(vec![value]));
        let batch = RecordBatch::try_new(schema.clone(), vec![keys, values]).unwrap();
        let file = File::create(path).unwrap();
        let props = WriterProperties::builder().set_writer_version(WriterVersion::PARQUET_2_0).build();
        let mut writer = ArrowWriter::try_new(file, schema, Some(props)).unwrap();
        writer.write(&batch).unwrap();
        writer.close().unwrap();
        println!("write succeed!")
    }
}

impl Reader for ParquetStorage {
    fn read_by_key(&self, key: String) {
        let path = Path::new("/Users/admin/Documents/code/aurach/data/sample.parquet");
        let file = File::open(path).unwrap();

        let file_reader = SerializedFileReader::new(file).unwrap();
        let mut arrow_reader = ParquetFileArrowReader::new(Arc::new(file_reader));

        println!("Converted arrow schema is: {}", arrow_reader.get_schema().unwrap());

        println!("Arrow schema after projection is: {}", arrow_reader.get_schema_by_columns(vec![0], true).unwrap());

        let mut record_batch_reader = arrow_reader.get_record_reader(2048).unwrap();

        for maybe_record_batch in record_batch_reader {
           let record_batch = maybe_record_batch.unwrap();
           if record_batch.num_rows() > 0 {
               let rows = record_batch.columns();
               for row in rows.iter() {
                   let r = column.as_any().downcast_ref::<StringArray>().expect("failed parse");
                   for r_str in r.iter() {
                       println!("{}", r_str.expect("failedc2"))
                   }
               }
               println!("Read {} records.", record_batch.num_rows());
           } else {
               println!("End of file!");
           }
        }

        println!("read succeed!")
    }
}