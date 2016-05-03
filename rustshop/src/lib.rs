#[macro_use]
extern crate mrusty;

use mrusty::*; 

pub struct Image {  
    pub width: i32,
    pub height: i32
}

impl Image {  
    pub fn new(width: i32, height: i32) -> Image {
        Image {
            width: width,
            height: height
        }
    }

    pub fn resolution(&self) -> i32 {
        self.width * self.height
    }
}

mrusty_class!(Image, {  
    // initialize needs to return the reflected type i.e. Image
    def!("initialize", |width: i32, height: i32| {
        Image::new(width, height)
    });

    // everything else needs to be converted to Ruby types
    def!("width", |mruby, slf: Image| {
        mruby.fixnum(slf.width)
    });

    def!("height", |mruby, slf: Image| {
        mruby.fixnum(slf.height)
    });

    // arguments to methods are automatically converted to Rust types
    def!("resolution", |mruby, slf: Image| {
        mruby.fixnum(slf.resolution())
    });
});

describe!(Image, "  
  context 'when 2 x 3' do
    subject { Image.new 2, 3 }

    it 'returns width' do
      expect(subject.width).to eql 2
    end

    it 'returns height' do
      expect(subject.height).to eql 3
    end

    it 'returns resolution' do
      expect(subject.resolution).to eql 6
    end
  end
");
