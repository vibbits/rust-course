use std::ffi::c_void;

extern "C" {
    fn create_resource(x: i32) -> *mut c_void;
    fn get_resource_value(r: *const c_void) -> i32;
    fn set_resource_value(r: *const c_void, val: i32);
    fn dispose_resource(r: *mut c_void);
}

pub struct Resource {
    ptr: *mut c_void
}

impl Resource {
    pub fn new(value: i32) -> Self {
        unsafe {
            Self {
                ptr: create_resource(value)          
            }    
        }
    }

    pub fn get(&self) -> i32 {
        unsafe {
            get_resource_value(self.ptr)
        }
    }

    pub fn set(&mut self, value: i32) {
        unsafe {
            set_resource_value(self.ptr, value)
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            dispose_resource(self.ptr)
        }
    }
}

fn main() {
    unsafe {
        let r = create_resource(3);
        println!("Got resource {}", get_resource_value(r));
        set_resource_value(r, 5);
        println!("Changed resource to {}", get_resource_value(r));        
        dispose_resource(r);
    }
    
    let mut r = Resource::new(3);
    println!("Got resource {}", r.get());
    r.set(6);
    let r = Box::new(r);
    println!("Changed resource to {}", r.get()); 
}
