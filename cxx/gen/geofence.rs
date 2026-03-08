// Auto-generated wrapper. Do not edit.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct PointOwned {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
}

/*  */
pub struct Point<'a> {
    inner: &'a crate::geofence::mavsdk::Geofence_Point,
}

impl<'a> Point<'a> {
    pub fn new(inner: &'a crate::geofence::mavsdk::Geofence_Point) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> PointOwned {
        PointOwned {
            latitude_deg: self.latitude_deg(),
            longitude_deg: self.longitude_deg(),
        }
    }
    ///  Latitude in degrees (range: -90 to +90)

    pub fn latitude_deg(&self) -> f64 {
        self.inner.geofence_get_latitude_deg()
    }
    ///  Longitude in degrees (range: -180 to +180)

    pub fn longitude_deg(&self) -> f64 {
        self.inner.geofence_get_longitude_deg()
    }
}

pub struct PolygonOwned {
    pub points: std::vec::Vec<PointOwned>,
    pub fence_type: crate::geofence::mavsdk::Geofence_FenceType,
}

/*  */
pub struct Polygon<'a> {
    inner: &'a crate::geofence::mavsdk::Geofence_Polygon,
}

impl<'a> Polygon<'a> {
    pub fn new(inner: &'a crate::geofence::mavsdk::Geofence_Polygon) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> PolygonOwned {
        PolygonOwned {
            points: self.points(),
            fence_type: self.fence_type(),
        }
    }
    ///  Points defining the polygon

    pub fn points(&self) -> std::vec::Vec<PointOwned> {
        self.inner
            .geofence_get_points()
            .iter()
            .map(|item| Point::new(item).into_owned())
            .collect()
    }
    ///  Fence type

    pub fn fence_type(&self) -> crate::geofence::mavsdk::Geofence_FenceType {
        self.inner.geofence_get_fence_type().clone()
    }
}

pub struct CircleOwned {
    pub point: PointOwned,
    pub radius: f32,
    pub fence_type: crate::geofence::mavsdk::Geofence_FenceType,
}

/*  */
pub struct Circle<'a> {
    inner: &'a crate::geofence::mavsdk::Geofence_Circle,
}

impl<'a> Circle<'a> {
    pub fn new(inner: &'a crate::geofence::mavsdk::Geofence_Circle) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> CircleOwned {
        CircleOwned {
            point: self.point(),
            radius: self.radius(),
            fence_type: self.fence_type(),
        }
    }
    ///  Point defining the center

    pub fn point(&self) -> PointOwned {
        Point::new(self.inner.geofence_get_point()).into_owned()
    }
    ///  Radius of the circular fence

    pub fn radius(&self) -> f32 {
        self.inner.geofence_get_radius()
    }
    ///  Fence type

    pub fn fence_type(&self) -> crate::geofence::mavsdk::Geofence_FenceType {
        self.inner.geofence_get_fence_type().clone()
    }
}

pub struct GeofenceDataOwned {
    pub polygons: std::vec::Vec<PolygonOwned>,
    pub circles: std::vec::Vec<CircleOwned>,
}

/*  */
pub struct GeofenceData<'a> {
    inner: &'a crate::geofence::mavsdk::Geofence_GeofenceData,
}

impl<'a> GeofenceData<'a> {
    pub fn new(inner: &'a crate::geofence::mavsdk::Geofence_GeofenceData) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> GeofenceDataOwned {
        GeofenceDataOwned {
            polygons: self.polygons(),
            circles: self.circles(),
        }
    }
    ///  Polygon(s) representing the geofence(s)

    pub fn polygons(&self) -> std::vec::Vec<PolygonOwned> {
        self.inner
            .geofence_get_polygons()
            .iter()
            .map(|item| Polygon::new(item).into_owned())
            .collect()
    }
    ///  Circle(s) representing the geofence(s)

    pub fn circles(&self) -> std::vec::Vec<CircleOwned> {
        self.inner
            .geofence_get_circles()
            .iter()
            .map(|item| Circle::new(item).into_owned())
            .collect()
    }
}

struct GeofenceInner {
    plugin: cxx::UniquePtr<crate::geofence::mavsdk::Geofence>,
}

pub struct GeofenceClient {
    inner: Mutex<GeofenceInner>,
}

impl GeofenceClient {
    pub fn new(plugin: cxx::UniquePtr<crate::geofence::mavsdk::Geofence>) -> Self {
        Self {
            inner: Mutex::new(GeofenceInner { plugin }),
        }
    }
}
