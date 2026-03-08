// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct IntParamOwned {
    pub name: String,
    pub value: i32,
}

/*  */
pub struct IntParam<'a> {
    inner: &'a crate::param::mavsdk::Param_IntParam,
}

impl<'a> IntParam<'a> {
    pub fn new(inner: &'a crate::param::mavsdk::Param_IntParam) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> IntParamOwned {
        IntParamOwned {
            name: self.name(),
            value: self.value(),
        }
    }
    ///  Name of the parameter

    pub fn name(&self) -> String {
        self.inner.param_get_name().to_string_lossy().into_owned()
    }
    ///  Value of the parameter

    pub fn value(&self) -> i32 {
        self.inner.param_get_value()
    }
}

pub struct FloatParamOwned {
    pub name: String,
    pub value: f32,
}

/*  */
pub struct FloatParam<'a> {
    inner: &'a crate::param::mavsdk::Param_FloatParam,
}

impl<'a> FloatParam<'a> {
    pub fn new(inner: &'a crate::param::mavsdk::Param_FloatParam) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> FloatParamOwned {
        FloatParamOwned {
            name: self.name(),
            value: self.value(),
        }
    }
    ///  Name of the parameter

    pub fn name(&self) -> String {
        self.inner.param_get_name().to_string_lossy().into_owned()
    }
    ///  Value of the parameter

    pub fn value(&self) -> f32 {
        self.inner.param_get_value()
    }
}

pub struct CustomParamOwned {
    pub name: String,
    pub value: String,
}

/*  */
pub struct CustomParam<'a> {
    inner: &'a crate::param::mavsdk::Param_CustomParam,
}

impl<'a> CustomParam<'a> {
    pub fn new(inner: &'a crate::param::mavsdk::Param_CustomParam) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> CustomParamOwned {
        CustomParamOwned {
            name: self.name(),
            value: self.value(),
        }
    }
    ///  Name of the parameter

    pub fn name(&self) -> String {
        self.inner.param_get_name().to_string_lossy().into_owned()
    }
    ///  Value of the parameter (max len 128 bytes)

    pub fn value(&self) -> String {
        self.inner.param_get_value().to_string_lossy().into_owned()
    }
}

pub struct AllParamsOwned {
    pub int_params: std::vec::Vec<IntParamOwned>,
    pub float_params: std::vec::Vec<FloatParamOwned>,
    pub custom_params: std::vec::Vec<CustomParamOwned>,
}

/*  */
pub struct AllParams<'a> {
    inner: &'a crate::param::mavsdk::Param_AllParams,
}

impl<'a> AllParams<'a> {
    pub fn new(inner: &'a crate::param::mavsdk::Param_AllParams) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> AllParamsOwned {
        AllParamsOwned {
            int_params: self.int_params(),
            float_params: self.float_params(),
            custom_params: self.custom_params(),
        }
    }
    ///  Collection of all parameter names and values of type int

    pub fn int_params(&self) -> std::vec::Vec<IntParamOwned> {
        self.inner
            .param_get_int_params()
            .iter()
            .map(|item| IntParam::new(item).into_owned())
            .collect()
    }
    ///  Collection of all parameter names and values of type float

    pub fn float_params(&self) -> std::vec::Vec<FloatParamOwned> {
        self.inner
            .param_get_float_params()
            .iter()
            .map(|item| FloatParam::new(item).into_owned())
            .collect()
    }
    ///  Collection of all parameter names and values of type custom

    pub fn custom_params(&self) -> std::vec::Vec<CustomParamOwned> {
        self.inner
            .param_get_custom_params()
            .iter()
            .map(|item| CustomParam::new(item).into_owned())
            .collect()
    }
}

struct ParamInner {
    plugin: cxx::UniquePtr<crate::param::mavsdk::Param>,
}

pub struct ParamClient {
    inner: Mutex<ParamInner>,
}

impl ParamClient {
    pub fn new(plugin: cxx::UniquePtr<crate::param::mavsdk::Param>) -> Self {
        Self {
            inner: Mutex::new(ParamInner { plugin }),
        }
    }
}
