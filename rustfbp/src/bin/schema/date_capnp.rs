// Generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: date.capnp


pub mod date {
  #![allow(unused_imports)]
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{text, data, Result};
  use capnp::private::layout;
  use capnp::traits::{FromStructBuilder, FromStructReader};
  use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};

  pub struct Owned;
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>
  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>
  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader : reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>
  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>) -> Result<Reader<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(try!(reader.get_struct(::std::ptr::null()))))
    }
  }

  impl <'a,> Reader<'a,>
  {
    pub fn borrow<'b>(&'b self) -> Reader<'b,> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_year(self) -> i16 {
      self.reader.get_data_field::<i16>(0)
    }
    #[inline]
    pub fn get_month(self) -> i8 {
      self.reader.get_data_field::<i8>(2)
    }
    #[inline]
    pub fn get_day(self) -> i8 {
      self.reader.get_data_field::<i8>(3)
    }
  }

  pub struct Builder<'a> { builder : ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>
  {
    #[inline]
    fn struct_size() -> layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>
   {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>
   {
    fn new(builder : ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder : builder,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>
   {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size : u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>) -> Result<Builder<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(try!(builder.get_struct(_private::STRUCT_SIZE, ::std::ptr::null()))))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>
   {
    fn set_pointer_builder<'b>(pointer : ::capnp::private::layout::PointerBuilder<'b>, value : Reader<'a,>) -> Result<()> { pointer.set_struct(&value.reader) }
  }

  impl <'a,> Builder<'a,>
   {
    pub fn as_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }
    pub fn borrow<'b>(&'b mut self) -> Builder<'b,> {
      Builder { .. *self }
    }
    pub fn borrow_as_reader<'b>(&'b self) -> Reader<'b,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.builder.as_reader().total_size()
    }
    #[inline]
    pub fn get_year(self) -> i16 {
      self.builder.get_data_field::<i16>(0)
    }
    #[inline]
    pub fn set_year(&mut self, value : i16)  {
      self.builder.set_data_field::<i16>(0, value);
    }
    #[inline]
    pub fn get_month(self) -> i8 {
      self.builder.get_data_field::<i8>(2)
    }
    #[inline]
    pub fn set_month(&mut self, value : i8)  {
      self.builder.set_data_field::<i8>(2, value);
    }
    #[inline]
    pub fn get_day(self) -> i8 {
      self.builder.get_data_field::<i8>(3)
    }
    #[inline]
    pub fn set_day(&mut self, value : i8)  {
      self.builder.set_data_field::<i8>(3, value);
    }
  }

  pub struct Pipeline { _typeless : ::capnp::any_pointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless,  }
    }
  }
  impl Pipeline {
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE : layout::StructSize = layout::StructSize { data : 1, pointers : 0 };
    pub const TYPE_ID: u64 = 0xd9ccffea5a9cf423;
  }
}
