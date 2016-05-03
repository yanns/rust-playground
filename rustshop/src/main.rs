extern crate mrusty;  
extern crate rustshop;

use std::path::Path;

use mrusty::*;

use rustshop::Image;

fn main() {  
    let mruby = Mruby::new();

    // we're placing the Image class in the virtual, requirable file 'image'
    mruby.def_file::<Image>("image");

    mruby.execute(Path::new("plugin.rb")).unwrap();
}
