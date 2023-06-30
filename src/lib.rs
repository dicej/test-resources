// start of "generated" bindings, i.e. hypothetical output of:
// wit_bindgen::generate!({
//     path: "resources.wit",
//     exported_resources: {
//         "x": MyX
//     }
// });

pub type Z = my::resources::types::Z;

pub trait Resources {
    fn add(a: &Z, b: &Z) -> Z;
    fn test();
}

#[doc(hidden)]
pub unsafe fn call_add<T: Resources>(arg0: i32, arg1: i32) -> i32 {
    T::add(&Z::from_handle(arg0, false), &Z::from_handle(arg1, false)).into_handle()
}

#[doc(hidden)]
pub unsafe fn call_test<T: Resources>() {
    T::test();
}

macro_rules! export_resources(($t:ident) => {
    const _: () = {
      #[doc(hidden)]
      #[export_name = "add"]
      #[allow(non_snake_case)]
      unsafe extern "C" fn __export_add(arg0: i32,arg1: i32,) -> i32 {
        call_add::<$t>(arg0,arg1,)
      }

    };

    const _: () = {
      #[doc(hidden)]
      #[export_name = "test"]
      #[allow(non_snake_case)]
      unsafe extern "C" fn __export_test() {
        call_test::<$t>()
      }

    };

});

#[allow(clippy::all)]
pub mod imports {
    pub struct Y {
        handle: i32,
        owned: bool,
    }

    impl Y {
        #[doc(hidden)]
        pub fn into_handle(self) -> i32 {
            std::mem::ManuallyDrop::new(self).handle
        }

        pub fn new(a: f64) -> Y {
            unsafe {
                #[link(wasm_import_module = "imports")]
                extern "C" {
                    #[link_name = "[constructor]y"]
                    fn wit_import(_: f64) -> i32;
                }

                Y {
                    handle: wit_import(a),
                    owned: true,
                }
            }
        }

        pub fn get_a(&self) -> f64 {
            unsafe {
                #[link(wasm_import_module = "imports")]
                extern "C" {
                    #[link_name = "[method]y.get-a"]
                    fn wit_import(_: i32) -> f64;
                }

                wit_import(self.handle)
            }
        }

        pub fn set_a(&self, a: f64) {
            unsafe {
                #[link(wasm_import_module = "imports")]
                extern "C" {
                    #[link_name = "[method]y.set-a"]
                    fn wit_import(_: i32, _: f64);
                }

                wit_import(self.handle, a)
            }
        }

        pub fn add(y: Y, a: f64) -> Y {
            unsafe {
                #[link(wasm_import_module = "imports")]
                extern "C" {
                    #[link_name = "[static]y.add"]
                    fn wit_import(_: i32, _: f64) -> i32;
                }

                Y {
                    handle: wit_import(y.into_handle(), a),
                    owned: true,
                }
            }
        }
    }

    impl Drop for Y {
        fn drop(&mut self) {
            unsafe {
                if self.owned {
                    #[link(wasm_import_module = "imports")]
                    extern "C" {
                        #[link_name = "[resource-drop-own]y"]
                        fn wit_import(_: i32);
                    }

                    wit_import(self.handle)
                } else {
                    #[link(wasm_import_module = "imports")]
                    extern "C" {
                        #[link_name = "[resource-drop-borrow]y"]
                        fn wit_import(_: i32);
                    }

                    wit_import(self.handle)
                }
            }
        }
    }
}

mod my {
    pub mod resources {
        pub mod types {
            pub struct Z {
                handle: i32,
                owned: bool,
            }

            impl Z {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: i32, owned: bool) -> Z {
                    Z { handle, owned }
                }

                #[doc(hidden)]
                pub fn into_handle(self) -> i32 {
                    std::mem::ManuallyDrop::new(self).handle
                }

                pub fn new(a: f64) -> Z {
                    unsafe {
                        #[link(wasm_import_module = "my:resources/types")]
                        extern "C" {
                            #[link_name = "[constructor]z"]
                            fn wit_import(_: f64) -> i32;
                        }

                        Z {
                            handle: wit_import(a),
                            owned: true,
                        }
                    }
                }

                pub fn get_a(&self) -> f64 {
                    unsafe {
                        #[link(wasm_import_module = "my:resources/types")]
                        extern "C" {
                            #[link_name = "[method]z.get-a"]
                            fn wit_import(_: i32) -> f64;
                        }

                        wit_import(self.handle)
                    }
                }
            }

            impl Drop for Z {
                fn drop(&mut self) {
                    unsafe {
                        if self.owned {
                            #[link(wasm_import_module = "my:resources/types")]
                            extern "C" {
                                #[link_name = "[resource-drop-own]z"]
                                fn wit_import(_: i32);
                            }

                            wit_import(self.handle)
                        } else {
                            #[link(wasm_import_module = "my:resources/types")]
                            extern "C" {
                                #[link_name = "[resource-drop-borrow]z"]
                                fn wit_import(_: i32);
                            }

                            wit_import(self.handle)
                        }
                    }
                }
            }
        }
    }
}

pub mod exports {
    #[allow(clippy::all)]
    pub mod exports {
        use super::super::MyX as RepX;

        pub(in super::super) trait TraitX: Sized {
            fn new(a: f64) -> Self;
            fn get_a(&self) -> f64;
            fn set_a(&self, a: f64);
            fn add(x: X, a: f64) -> X;
        }

        pub(in super::super) struct X {
            handle: i32,
        }

        impl X {
            #[doc(hidden)]
            pub fn into_handle(self) -> i32 {
                std::mem::ManuallyDrop::new(self).handle
            }

            pub fn new(x: RepX) -> X {
                unsafe {
                    #[link(wasm_import_module = "[export]exports")]
                    extern "C" {
                        #[link_name = "[resource-new]x"]
                        fn wit_import(_: i32) -> i32;
                    }

                    X {
                        handle: wit_import(
                            std::mem::transmute::<*mut RepX, isize>(Box::into_raw(Box::new(x)))
                                .try_into()
                                .unwrap(),
                        ),
                    }
                }
            }
        }

        impl std::ops::Deref for X {
            type Target = RepX;

            fn deref(&self) -> &RepX {
                unsafe {
                    #[link(wasm_import_module = "[export]exports")]
                    extern "C" {
                        #[link_name = "[resource-rep]x"]
                        fn wit_import(_: i32) -> i32;
                    }

                    std::mem::transmute::<isize, &RepX>(wit_import(self.handle).try_into().unwrap())
                }
            }
        }

        #[doc(hidden)]
        #[export_name = "exports#[constructor]x"]
        #[allow(non_snake_case)]
        unsafe extern "C" fn __export_x_constructor(arg0: f64) -> i32 {
            X::new(RepX::new(arg0)).into_handle()
        }

        #[doc(hidden)]
        #[export_name = "exports#[method]x.get-a"]
        #[allow(non_snake_case)]
        unsafe extern "C" fn __export_x_get_a(arg0: i32) -> f64 {
            std::mem::transmute::<isize, &X>(arg0.try_into().unwrap()).get_a()
        }

        #[doc(hidden)]
        #[export_name = "exports#[method]x.set-a"]
        #[allow(non_snake_case)]
        unsafe extern "C" fn __export_x_set_a(arg0: i32, arg1: f64) {
            std::mem::transmute::<isize, &X>(arg0.try_into().unwrap()).set_a(arg1)
        }

        #[doc(hidden)]
        #[export_name = "exports#[static]x.add"]
        #[allow(non_snake_case)]
        unsafe extern "C" fn __export_x_add(arg0: i32, arg1: f64) -> i32 {
            RepX::add(X { handle: arg0 }, arg1).into_handle()
        }

        #[doc(hidden)]
        #[export_name = "exports#[dtor]x"]
        #[allow(non_snake_case)]
        unsafe extern "C" fn __export_x_dtor(arg0: i32) {
            drop(Box::from_raw(std::mem::transmute::<isize, *mut RepX>(
                arg0.try_into().unwrap(),
            )))
        }
    }
}

// (end of "generated" bindings)

use {
    exports::exports::{TraitX, X},
    imports::Y,
    std::cell::RefCell,
};

struct MyResources;

impl Resources for MyResources {
    fn add(a: &Z, b: &Z) -> Z {
        Z::new(a.get_a() + b.get_a())
    }

    fn test() {
        let y = Y::new(42.0);
        assert_eq!(42.0, y.get_a());

        y.set_a(43.0);
        assert_eq!(43.0, y.get_a());

        let y = Y::add(Y::new(44.0), 45.0);
        assert_eq!(89.0, y.get_a());
    }
}

export_resources!(MyResources);

struct MyX(RefCell<f64>);

impl TraitX for MyX {
    fn new(a: f64) -> Self {
        Self(RefCell::new(a))
    }

    fn get_a(&self) -> f64 {
        *self.0.borrow()
    }

    fn set_a(&self, a: f64) {
        *self.0.borrow_mut() = a;
    }

    fn add(x: X, a: f64) -> X {
        x.set_a(a + x.get_a());
        x
    }
}
